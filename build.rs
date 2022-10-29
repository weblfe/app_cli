use std::env;
use std::str;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

extern crate cc;

const TPL: &[u8; 60] = b"
    pub fn commit_id() -> &'static str {
        \"{}\"
    }";

fn pack_commit_id() {
    let result = Command::new("git").args(&["rev-parse", "--short", "HEAD"]).output().unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("git.rs");
    let mut f = File::create(&dest_path).unwrap();
    let hash_id = str::from_utf8(&result.stdout).unwrap();
    let code_str = str::from_utf8(TPL).unwrap();
    let code = code_str.replace("{}",hash_id);
    f.write_all(code.as_bytes()).unwrap();
}

fn main() {
    pack_commit_id();
    cc::Build::new()
        .file("src/c/git.c")
        .compile("git");
}