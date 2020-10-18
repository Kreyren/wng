use colored::*;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Command;

pub fn run(args: Vec<&str>) -> std::io::Result<()> {
    let debug = ".\\build\\debug\\debug.exe";
    let release = ".\\build\\release\\release.exe";

    if Path::new(debug).exists() && !Path::new(release).exists() {
        let status = Command::new(debug)
            .args(args)
            .status()
            .expect("Cannot run binary");
        if status.code() != Some(0) {
            println!(
                "{}{}",
                "Process didn't exit successfully, exit code : ".red(),
                status.code().unwrap_or(i32::MAX)
            );
        }
        fs::remove_file(debug)?;
        Ok(())
    } else if Path::new(release).exists() && !Path::new(debug).exists() {
        let status = Command::new(release)
            .args(args)
            .status()
            .expect("Cannot read binary");
        if status.code() != Some(0) {
            println!(
                "{}, exit code : {}",
                "Process didn't exit successfully".red(),
                status.code().unwrap_or(i32::MAX)
            );
        }
        fs::remove_file(release)?;
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Cannot find binary"))
    }
}
