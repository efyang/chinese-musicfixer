use std::path::PathBuf;
use std::process::Command;
use simplet2s;

#[derive(Debug)]
pub enum AudioFile {
    MP3(PathBuf),
    M4A(PathBuf),
    FLAC(PathBuf),
}

pub fn into_audio_file(p: PathBuf) -> Option<AudioFile> {
    match p.extension().unwrap().to_str().unwrap() {
        "mp3" => Some(AudioFile::MP3(p)),
        "m4a" => Some(AudioFile::M4A(p)),
        "flac" => Some(AudioFile::FLAC(p)),
        _ => None
    }
}

impl AudioFile {
    pub fn simplify_metadata(&self) {
        match *self {
            AudioFile::FLAC(ref p) => {
                if let Ok(mut tag) = ::metaflac::Tag::read_from_path(p) {
                    println!("Retagging {}", p.to_str().unwrap());
                    simplify_vorbis_tag(&mut tag, "TITLE");
                    simplify_vorbis_tag(&mut tag, "ALBUM");
                    simplify_vorbis_tag(&mut tag, "ARTIST");

                    let artist_album = tag.get_vorbis("ARTISTALBUM")
                        .or(tag.get_vorbis("ARTIST ALBUM"))
                        .cloned();
                    let simplified_artist_album = artist_album
                        .map(|strv| strv
                             .iter()
                             .map(|s| simplet2s::convert(s))
                             .collect::<Vec<_>>());
                    simplified_artist_album.map(|v| tag.set_vorbis("ARTISTALBUM", v));

                    tag.save().unwrap();
                }
            }
            AudioFile::MP3(ref p) => {
                if let Ok(mut tag) = ::id3::Tag::read_from_path(p) {
                    println!("Retagging {}", p.to_str().unwrap());
                    tag.title().map(|s| s.to_string()).map(|s| tag.set_title(simplet2s::convert(&s)));
                    tag.artist().map(|s| s.to_string()).map(|s| tag.set_artist(simplet2s::convert(&s)));
                    tag.album().map(|s| s.to_string()).map(|s| tag.set_album(simplet2s::convert(&s)));
                    tag.album_artist().map(|s| s.to_string()).map(|s| tag.set_album_artist(simplet2s::convert(&s)));
                }
            }
            AudioFile::M4A(ref p) => {
                println!("Retagging {}", p.to_str().unwrap());
                // use python libs
                simplify_m4a_tag(p, "title");
            }
        }
    }
}

fn simplify_vorbis_tag(tag: &mut ::metaflac::Tag, name: &str) {
    let value = tag.get_vorbis(name).cloned();
    let simplified_value = value
        .map(|strv| strv
             .iter()
             .map(|s| simplet2s::convert(s))
             .collect::<Vec<_>>());
    simplified_value.map(|v| tag.set_vorbis(name, v));
}

fn simplify_m4a_tag(path: &PathBuf, name: &str) {
    let py_path = ::std::env::current_exe().unwrap().parent().unwrap().join("m4a.py");
    let _ = Command::new("python")
        .arg(py_path.clone())
        .arg(path)
        .arg(name)
        .output()
        .map(|output| {
            let simplified_value = simplet2s::convert(String::from_utf8_lossy(&output.stdout).trim());
            Command::new("python")
                .arg(py_path)
                .arg(path)
                .arg(name)
                .arg("--set-value")
                .arg(simplified_value)
                .spawn()
                .expect("Failed to set value")
        });
}
