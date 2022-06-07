extern crate cmd_utils;

use std::fs::{File, self};
use std::process::Command;
use cmd_utils::CmdToFile;

fn main() {
    if let Err(e) = fs::create_dir("tmp") {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("{}", e),
        }
    }
    let stdout = File::create("tmp/my_file.stdout").unwrap();
    let stderr = File::create("tmp/my_file.stderr").unwrap();
    let mut cmd = Command::new("echo");
    // writes stdout to file, and optional stderr to another file
    cmd.arg("test").to_file(stdout, Some(stderr)).unwrap();
}