use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;

#[allow(unused_assignments)]
pub fn run(args: Vec<&str>) -> Result<(), String> {
    let mut debug = "";
    let mut release = "";

    if cfg!(windows) {
        debug = ".\\build\\debug\\debug.exe";
        release = ".\\build\\release\\release.exe";
    } else {
        debug = "./build/debug/debug.exe";
        release = "./build/release/release.exe";
    }

    if Path::new(debug).exists() && !Path::new(release).exists() {
        let status = Command::new(debug)
            .args(args)
            .status()
            .expect("Cannot run binary");
        if status.code() != Some(0) {
            return Err(format!(
                "{}{}",
                "Process didn't exit successfully, exit code : ".red(),
                status.code().unwrap_or(i32::MAX)
            ));
        }
        match fs::remove_file(debug) {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        Ok(())
    } else if Path::new(release).exists() && !Path::new(debug).exists() {
        let status = match Command::new(release).args(args).status() {
            Ok(s) => s,
            Err(e) => return Err(format!("{}", e)),
        };
        if status.code() != Some(0) {
            return Err(format!(
                "{}, exit code : {}",
                "Process didn't exit successfully".red(),
                status.code().unwrap_or(i32::MAX)
            ));
        }
        match fs::remove_file(release) {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        };
        Ok(())
    } else {
        Err("Cannot find binary".to_owned())
    }
}
