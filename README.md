![Crates.io](https://img.shields.io/crates/v/cmd-utils?style=for-the-badge)
![Docs.rs](https://img.shields.io/docsrs/cmd-utils/0.2.0?style=for-the-badge)

# Cmd Utils

Safe Rust `Command` utility traits
- run command (spawn & wait wrapper)
- pipe commands
- output to file (spawn & wait & outuput to file wrapper)

## CmdRun trait
```rust
use std::process::Command;
use cmd_utils::CmdRun;

// `spawn` the command and `wait` for child process to end
// note that `run` does not return the command output, just Ok(())
// but you can redirect stdout & stderr where you want
Command::new("test")
    // .stdout(cfg)
    // .stderr(cfg)
    .args(["-n", "a"])
    .run()
    .unwrap();
```

example available:
```bash
cargo run --example cmd_run
```

## CmdPipe trait
```rust
use std::process::Command;
use std::str;
use cmd_utils::CmdPipe;

// pipe echo & wc commands
// equivalent to bash: echo test | wc -c
let mut echo = Command::new("echo");
let mut wc = Command::new("wc");
let output = echo.arg("test")
    .pipe(&mut wc.arg("-c"))
    .unwrap();
let res = str::from_utf8(&output.stdout).unwrap();
println!("pipe result: {}", &res);
```

example available:
```bash
cargo run --example cmd_pipe
```

## CmdToFile trait
```rust
use std::fs::File;
use std::process::Command;
use cmd_utils::CmdToFile;

let stdout = File::create("my_file.stdout").unwrap();
let stderr = File::create("my_file.stderr").unwrap();
let mut cmd = Command::new("echo");
// writes stdout to file
cmd.arg("test").to_file(stdout, Some(stderr));
```

example available:
```bash
cargo run --example cmd_to_file
```
