use std::env;
use std::path::Path;
use std::process::Command;
use std::fs;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    fs::copy("src/m4a.py", Path::new(&out_dir).join("../../../m4a.py")).unwrap();
    fs::copy("src/id3.py", Path::new(&out_dir).join("../../../id3.py")).unwrap();

    let _ = Command::new("pip")
        .arg("install")
        .arg("--user")
        .arg("mutagen")
        .spawn();
}
