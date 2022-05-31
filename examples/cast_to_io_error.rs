extern crate cmd_utils;

use std::process::Command;
use cmd_utils::CmdRun;

fn main() {
    match run() {
        Ok(_) => todo!(),
        Err(e) => {
            eprintln!("error: {}", e)
        },
    };
}

fn run() -> Result<(), std::io::Error> {
    Command::new("test")
        .args(["-n", ""])
        .run()?;
    Ok(())
}