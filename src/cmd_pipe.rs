use std::{io, fs::File};
use io::{BufWriter, Write, BufReader, BufRead};
use std::process::{Command, Stdio, Output};

/// Pipe **stdout** of `command` to **stdin** of `piped` command,
/// and pipe **stdout** of `piped` to `piped_stdout`
///
/// # Errors
///
/// command_pipe can result in `std::io::Error`
/// - when `spawn` or `wait` fail
/// - when there is an issue with the **stdout** / **stdin** pipe (std::io::ErrorKind::BrokenPipe)
#[allow(clippy::manual_flatten)]
pub fn command_pipe_base<T> (
    command: &mut Command, piped: &mut Command, piped_stdout: T
) -> Result<Output, io::Error> where T: Into<Stdio>{
    let process = command.stdout(Stdio::piped()).spawn();
    match process {
        Ok(child) => {
            if let Some(stdout) = child.stdout {
                let piped_process = piped
                    .stdin(Stdio::piped())
                    .stdout(piped_stdout)
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

/// Pipe **stdout** of `command` to **stdin** of `piped` command
///
/// # Errors
///
/// command_pipe can result in `std::io::Error`
/// - when `spawn` or `wait` fail
/// - when there is an issue with the **stdout** / **stdin** pipe (std::io::ErrorKind::BrokenPipe)
pub fn command_pipe (
    command: &mut Command, piped: &mut Command
) -> Result<Output, io::Error> {
    return command_pipe_base(command, piped, Stdio::piped());
}

/// Pipe **stdout** of `command` to **stdin** of `piped` command
/// and pipe **stdout** of `piped` to `file`
///
/// # Errors
///
/// command_pipe_to_file can result in `std::io::Error`
/// - when `spawn` or `wait` fail
/// - when there is an issue with the **stdout** / **stdin** pipe (std::io::ErrorKind::BrokenPipe)
pub fn command_pipe_to_file (
    command: &mut Command, piped: &mut Command, file: File
) -> Result<(), io::Error> {
    return match command_pipe_base(command, piped, file) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}

pub trait CmdPipe {
    /// Pipe **stdout** of `self` to **stdin** of `piped_command`
    ///
    /// # Errors
    ///
    /// command.pipe(cmd) can result in `std::io::Error`
    /// - when `spawn` or `wait` fail
    /// - when there is an issue with the **stdout** / **stdin** pipe (std::io::ErrorKind::BrokenPipe)
    fn pipe (&mut self, piped_command: &mut Command) -> Result<Output, io::Error>;

    /// Pipe **stdout** of `self` to **stdin** of `piped_command`,
    /// and pipe **stdout** of `piped_command` to `file`
    /// # Errors
    ///
    /// command.pipe_to_file(cmd, file) can result in `std::io::Error`
    /// - when `spawn` or `wait` fail
    /// - when there is an issue with the **stdout** / **stdin** pipe (std::io::ErrorKind::BrokenPipe)
    fn pipe_to_file (&mut self, piped_command: &mut Command, file: File) -> Result<(), io::Error>;
}

impl CmdPipe for Command {
    fn pipe(&mut self, piped_command: &mut Command) -> Result<Output, io::Error> {
        command_pipe(self, piped_command)
    }

    fn pipe_to_file (&mut self, piped_command: &mut Command, file: File) -> Result<(), io::Error> {
        command_pipe_to_file(self, piped_command, file)
    }
}

// TODO: allow piping multiple commands
// pub fn pipe_commands (commands: Vec<&mut Command>) -> Result<Output, io::Error>{
//     todo!()
// }

#[cfg(test)]
mod test {
    use std::fs;
    use std::io::Read;
    use std::process::Command;
    use std::str::from_utf8;

    use super::CmdPipe;

    use super::command_pipe;

    #[test]
    fn test_command_to_pipe () {
        let mut echo = Command::new("echo");
        let mut wc = Command::new("wc");
        let output = command_pipe(
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

    #[test]
    fn test_command_pipe_to_file () {
        const FILE_NAME: &str = "tmp/__command_pipe_to_file";
        let mut echo = Command::new("echo");
        let mut wc = Command::new("wc");
        if let Err(e) = fs::create_dir("tmp") {
            match e.kind() {
                std::io::ErrorKind::AlreadyExists => (),
                _ => panic!("{}", e),
            }
        }
        let file = fs::File::create(FILE_NAME).unwrap();
        echo.args(["-n", "test"])
            .pipe_to_file(&mut wc.arg("-c"), file)
            .unwrap();

        // read result
        let mut read_file = fs::File::open(FILE_NAME).unwrap();
        let mut res = String::new();
        match read_file.read_to_string(&mut res) {
            Err(e) => panic!("{}", e),
            _ => (),
        };

        assert_eq!(str::trim_start(&res), "5\n");

        // cleanup file
        fs::remove_file(FILE_NAME).unwrap();
    }
}
