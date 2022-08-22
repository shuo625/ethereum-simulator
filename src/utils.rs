pub mod solc {
    use std::{error::Error, path::Path, process::Command};

    const COMPILER: &'static str = "solc";

    pub fn compile(file: &Path) -> Result<String, Box<dyn Error>> {
        if !file.exists() {
            Err("file does not exist")?;
        }
        let command = Command::new(COMPILER)
            .arg("--bin")
            .arg(file.to_str().unwrap())
            .output()?;
        let output = String::from_utf8(command.stdout).unwrap();
        let lines: Vec<&str> = output.lines().collect();

        Ok(String::from(lines[3]))
    }
}

pub mod path {
    use std::path::Path;

    pub fn get_file_name(file_path: &Path) -> &str {
        Path::new(file_path.file_name().unwrap())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
    }
}
