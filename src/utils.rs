pub mod solc {
    use std::{error::Error, path::PathBuf, process::Command};

    const COMPILER: &'static str = "solc";

    pub fn compile(file: PathBuf) -> Result<String, Box<dyn Error>> {
        let command = Command::new(COMPILER).arg("--bin").arg(file);
        let output = String::from_utf8(command.output()?.stdout).unwrap();
        let lines: Vec<&str> = output.lines().collect();

        Ok(String::from(lines[3]))
    }
}
