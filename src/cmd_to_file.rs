use std::process::Command;
use std::fs::File;

use crate::{CmdRun, CmdSpawnError};

/// `spawn`, `wait` and send the stdout of the command to a file
///
/// # Errors
///
/// command_to_file can result in `CmdSpawnError`:
/// - `CmdSpawnError::IO(std::io::Error)` when `spawn` or `wait` fail
/// - `CmdSpawnError::Child(ChildError)` when the child process exit with a failed status
pub fn command_to_file (
    command: &mut Command,
    file: File,
    stderr_file: Option<File>
) -> Result<(), CmdSpawnError> {
    if let Some(f) = stderr_file {
        command.stderr(f);
    }
    command.stdout(file).run()
}

pub trait CmdToFile {
    /// `spawn`, `wait` and **stdout** to `file` for the child process,
    /// optional **stderr** to `stderr_file`
    ///
    /// # Errors
    ///
    /// command.to_file(file, err_file) can result in `CmdSpawnError`:
    /// - `CmdSpawnError::IO(std::io::Error)` when `spawn` or `wait` fail
    /// - `CmdSpawnError::Child(ChildError)` when the child process exit with a failed status
    fn to_file(&mut self, file: File, stderr_file: Option<File>) -> Result<(), CmdSpawnError>;
}

impl CmdToFile for Command {
    fn to_file(&mut self, file: File, stderr_file: Option<File>) -> Result<(), CmdSpawnError> {
        command_to_file(self, file, stderr_file)
    }
}

#[cfg(test)]
mod tests {
    use super::CmdToFile;
    use std::{fs::{File, remove_file}, io::Read};
    use std::process::Command;

    #[test]
    fn to_file() {
        let file_location = ".to_file_test";
        let file_content = "file-content-test";
        let file = File::create(&file_location).unwrap();
        let mut cmd = Command::new("echo");
        match cmd.arg(&file_content).to_file(file, None) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };

        // read file
        let mut file = File::open(&file_location).unwrap();
        let mut res = String::new();
        match file.read_to_string(&mut res) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };

        assert_eq!(res, file_content.to_owned() + "\n");

        // cleanup file
        remove_file(&file_location).unwrap();
    }
}
