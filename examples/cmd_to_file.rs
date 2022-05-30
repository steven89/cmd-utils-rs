extern crate cmd_utils;

use std::fs::File;
use std::process::Command;
use cmd_utils::CmdToFile;

fn main() {
    let stdout = File::create("my_file.stdout").unwrap();
    let stderr = File::create("my_file.stderr").unwrap();
    let mut cmd = Command::new("echo");
    // writes stdout to file, and optional stderr to another file
    cmd.arg("test").to_file(stdout, Some(stderr)).unwrap();
}