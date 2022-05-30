use std::io;
use io::{BufWriter, Write, BufReader, BufRead};
use std::process::{Command, Stdio, Output};

/// Pipe **stdout** of `command` to **stdin** of `piped` command
#[allow(clippy::manual_flatten)]
pub fn command_to_pipe (command: &mut Command, piped: &mut Command) -> Result<Output, io::Error> {
    let process = command.stdout(Stdio::piped()).spawn();
    match process {
        Ok(child) => {
            if let Some(stdout) = child.stdout {
                let piped_process = piped
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn();
                match piped_process {
                    Ok(mut piped_child) => {
                        if let Some(mut stdin) = piped_child.stdin.take() {
                            let mut writer = BufWriter::new(&mut stdin);
                            for line in BufReader::new(stdout).lines() {
                                if let Ok(l) = line {
                                    writer.write_all(l.as_bytes()).unwrap();
                                    writer.write_all(&[b'\n']).unwrap();
                                }
                            }
                        } else {
                            return Err(io::Error::new(
                                io::ErrorKind::BrokenPipe,
                                "Could not pipe command, stdin not found"
                            ))
                        }
                        if let Ok(out) = piped_child.wait_with_output() {
                            return Ok(out)
                        } else {
                            return Err(io::Error::new(
                                io::ErrorKind::BrokenPipe,
                                "Could not wait for pipe"
                            ))
                        }
                    },
                    Err(e) => return Err(e),
                }
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "Could not pipe command, stdout not found"
                ))
            }
        },
        Err(e) => return Err(e),
    }
}

pub trait CmdPipe {
    /// Pipe **stdout** of `self` to **stdin** of `piped_command`
    fn pipe (&mut self, piped_command: &mut Command) -> Result<Output, io::Error>;
}

impl CmdPipe for Command {
    fn pipe(&mut self, piped_command: &mut Command) -> Result<Output, io::Error> {
        command_to_pipe(self, piped_command)
    }
}

// TODO: allow piping multiple commands
// pub fn pipe_commands (commands: Vec<&mut Command>) -> Result<Output, io::Error>{
//     todo!()
// }

#[cfg(test)]
mod test {
    use std::process::Command;
    use std::str::{from_utf8};

    use super::CmdPipe;

    use super::command_to_pipe;

    #[test]
    fn test_command_to_pipe () {
        let mut echo = Command::new("echo");
        let mut wc = Command::new("wc");
        let output = command_to_pipe(
            &mut echo.args(["-n", "test"]),
            &mut wc.arg("-c")
        ).unwrap();
        let res = match from_utf8(&output.stdout) {
            Ok(s) => s,
            Err(_) => panic!("unexpected stdout"),
        };
        // MacOs puts whitespace in from of wc result
        assert_eq!(str::trim_start(res), "5\n");
    }

    #[test]
    fn test_command_pipe_trait () {
        let mut echo = Command::new("echo");
        let mut wc = Command::new("wc");
        let output = echo.args(["-n", "test"])
            .pipe(&mut wc.arg("-c"))
            .unwrap();
        let res = match from_utf8(&output.stdout) {
            Ok(s) => s,
            Err(_) => panic!("unexpected stdout"),
        };
        // MacOs puts whitespace in from of wc result
        assert_eq!(str::trim_start(res), "5\n");
    }
}
