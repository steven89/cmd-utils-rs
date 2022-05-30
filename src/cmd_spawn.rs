use std::process::Command;
use std::io;
use std::fmt;

/// Child process error
#[derive(Debug)]
pub struct ChildError {
    /// Name of the child program, see [std::process::Command.get_program()](https://doc.rust-lang.org/std/process/struct.Command.html#method.get_program)
    pub program: String,
    /// Exit code of the child process see [std::process::ExitStatus.code()](https://doc.rust-lang.org/std/process/struct.ExitStatus.html#method.code)
    pub code: Option<i32>
}

impl fmt::Display for ChildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "program \"{}\" failed with status code {}",
            self.program,
            match self.code {
                Some(code) => code.to_string(),
                None => String::from("unknown")
            }
        )
    }
}

/// Error result of a command spawn
#[derive(Debug)]
pub enum CmdSpawnError {
    /// command spawn std::io error [std::io::Error](https://doc.rust-lang.org/std/io/struct.Error.html)
    IO(io::Error),
    /// Child process exited with error
    ChildError(ChildError)
}

impl fmt::Display for CmdSpawnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            CmdSpawnError::IO(e) => write!(f, "command IO error {}", e),
            CmdSpawnError::ChildError(e) => write!(f, "child {}", e),
        }
    }
}

/// `spawn` and `wait` command
pub fn command_spawn (command: &mut Command) -> Result<(), CmdSpawnError> {
    let process = command.spawn();
    match process {
        Ok(mut child) => {
            match child.wait() {
                Ok(status) => {
                    if !status.success() {
                        return Err(
                            CmdSpawnError::ChildError(
                                ChildError {
                                    program: command.get_program()
                                        .to_str()
                                        .unwrap_or("unknwown")
                                        .to_string(),
                                    code: status.code()
                                }
                            )
                        )
                    }
                    return Ok(())
                },
                Err(e) => return Err(CmdSpawnError::IO(e))
            };
        },
        Err(e) => return Err(CmdSpawnError::IO(e))
    }
}

pub trait CmdRun {
    /// `spawn` and `wait` child process
    fn run(&mut self) -> Result<(), CmdSpawnError>;
}

impl CmdRun for Command {
    fn run(&mut self) -> Result<(), CmdSpawnError> {
        command_spawn(self)
    }
}

#[cfg(test)]
mod tests {
    use super::CmdRun;
    use std::process::Command;

    #[test]
    fn cmd_spawn_success() {
        let mut cmd = Command::new("test");
        match cmd.args([
            "-n",
            "a"
        ]).run() {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };
    }

    #[test]
    fn cmd_spawn_child_error() {
        let program = "test";
        let mut cmd = Command::new(&"test");
        match cmd.args([
            "-n",
            ""
        ]).run() {
            Ok(_) => panic!("should not have succeded"),
            Err(e) => {
                match e {
                    crate::CmdSpawnError::IO(e) => panic!("{}", e),
                    crate::CmdSpawnError::ChildError(e) => {
                        assert_eq!(e.program, program.to_owned());
                        assert_eq!(e.code, Some(1 as i32));
                    },
                }
            },
        };
    }
}