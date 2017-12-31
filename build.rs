use std::env;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::fs;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    fs::copy("src/m4a.py", Path::new(&out_dir).join("m4a.py"));
    let _ = Command::new("pip")
        .arg("install")
        .arg("--user")
        .arg("mutagen");
}
