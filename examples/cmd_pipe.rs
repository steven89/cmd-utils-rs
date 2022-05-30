extern crate cmd_utils;

use std::process::Command;
use std::str;
use cmd_utils::CmdPipe;

fn main() {
    let mut echo = Command::new("echo");
    let mut wc = Command::new("wc");
    let output = echo.args(["-n", "test"])
        .pipe(&mut wc.arg("-c"))
        .unwrap();
    let res = str::from_utf8(&output.stdout).unwrap();
    println!("pipe result: {}", &res);
}