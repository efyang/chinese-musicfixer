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
use std::path::Path;
use failure::Error;

mod audio_types;
use audio_types::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "Chinese Music Fixer")]
struct Cli {
    source_dir: String,
    playlist_dir: String,
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
        eprintln!("Error: dir \"{}\" does not exist", args.source_dir)
    }

    for file in audio_file_paths {
        file.simplify_metadata();
    }
}

