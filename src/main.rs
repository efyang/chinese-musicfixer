extern crate m3u;
extern crate simplet2s;
extern crate failure;
extern crate structopt;
extern crate walkdir;
extern crate id3;
extern crate metaflac;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate structopt_derive;

use structopt::StructOpt;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use failure::Error;
use std::fs::File;

mod audio_types;
use audio_types::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "Chinese Music Fixer")]
struct Cli {
    source_dir: String,
    playlist_dir: String,
    strip_dir: String,
}

fn main() {
    let args = Cli::from_args();
    let source_dir = Path::new(&args.source_dir);
    let mut audio_file_paths = Vec::new();
    if source_dir.exists() {
        // first fix all of the filenames
        for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()) {
            let file_name = entry.file_name().to_str().unwrap();
            let simplified_file_name = simplet2s::convert(file_name);
            if file_name != simplified_file_name {
                println!("Conflict: Renaming. \"{}\" -> \"{}\"", file_name, simplified_file_name);
                ::std::fs::rename(entry.path(), entry.path().parent().unwrap().join(simplified_file_name)).unwrap();
            }

            // add any audio files to our queue
            if entry.file_type().is_file() {
                into_audio_file(entry.path().to_path_buf()).map(|af| audio_file_paths.push(af));
            }
        }
    } else {
        eprintln!("Error: dir \"{}\" does not exist", args.source_dir);
    }

    for file in audio_file_paths {
        file.simplify_metadata();
    }

    let playlist_dir = Path::new(&args.playlist_dir);
    if playlist_dir.exists() {
        println!("Rewriting playlists");
        for entry in WalkDir::new(playlist_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file() && e.path().extension().unwrap() == "m3u") {
            let mut reader = m3u::Reader::open(entry.path()).unwrap();
            let mut playlist_iter = reader.entries()
                .map(|entry| entry.unwrap())
                .filter_map(|e| {
                    match e {
                        m3u::Entry::Path(p) => Some(p),
                        _ => None
                    }
                });

            // assume that playlist files are going to be based in a playlists folder in the music
            // folder
            let mut new_playlist = Vec::new();
            let base_dir = Path::new("..");
            for audio in playlist_iter {
                let reformed_path = audio.strip_prefix(&args.strip_dir).unwrap_or(&audio);
                new_playlist.push(simplify_path(reformed_path.to_path_buf()));
            }
            let mut f = File::create(entry.path()).unwrap();
            let mut writer = m3u::Writer::new(&mut f);
            for song in new_playlist {
                writer.write_entry(&m3u::Entry::Path(song)).unwrap();
            }
        }
        println!("Done");
    } else {
        eprintln!("Error: dir \"{}\" does not exist", args.playlist_dir);
    }
}

fn simplify_path(p: PathBuf) -> PathBuf {
    let mut simplified_path = PathBuf::new();
    for component in p.iter() {
        simplified_path.push(simplet2s::convert(component.to_str().unwrap()));
    }
    simplified_path
}
