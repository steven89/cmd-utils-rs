extern crate cmd_utils;

use std::fs::{File, self};
use std::process::Command;
use cmd_utils::CmdPipe;

fn main() {
    if let Err(e) = fs::create_dir("tmp") {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("{}", e),
        }
    }
    let stdout = File::create("tmp/piped.stdout").unwrap();
    let mut echo = Command::new("echo");
    let mut wc = Command::new("wc");
    echo.args(["-n", "test"])
        .pipe_to_file(&mut wc.arg("-c"), stdout)
        .unwrap();
}