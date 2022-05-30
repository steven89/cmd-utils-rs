extern crate cmd_utils;

use std::process::Command;
use cmd_utils::CmdRun;

fn main() {
    Command::new("test")
        .args(["-n", "a"])
        .run()
        .unwrap();
}
