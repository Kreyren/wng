use colored::*;
use lines_from_file::lines_from_file;
use serde_json::*;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn see_dir(dir: PathBuf, cpp: bool) -> Vec<PathBuf> {
    let mut list: Vec<PathBuf> = Vec::new();
    for entry in match std::fs::read_dir(dir.clone()) {
        Ok(e) => e,
        Err(_s) => {
            eprintln!("Failed to read src/");
            std::process::exit(66);
        }
    } {
        let entry = match entry {
            Ok(e) => e,
            Err(_e) => {
                eprintln!("Failed to read src/");
                std::process::exit(69);
            }
        };
        if entry.path().is_dir() {
            let sub: Vec<PathBuf> = see_dir(entry.path(), cpp);
            list.extend(sub);
        } else {
            if !cpp {
                if entry.path().extension().unwrap() == "c" {
                    list.push(entry.path().to_owned());
                }
            } else {
                if entry.path().extension().unwrap() == "cpp" {
                    list.push(entry.path().to_owned());
                }
            }
        }
    }
    list
}

pub fn removebinary() {
    match std::fs::remove_file("build/debug/debug.exe") {
        Ok(_) => (),
        Err(e) => {
            // Because if it is equal to NotFound that would tell that the file hasn't been compiled
            if e.kind() == ErrorKind::NotFound {
                std::process::exit(-1);
            }
            eprintln!("{:?}", e.kind());
            std::process::exit(58);
        }
    };
}

pub fn build(cpp: bool) {
    if !Path::new("src").exists() {
        eprintln!("src/ folder not found. Make sure to be in a valid project");
        std::process::exit(36);
    }
    let files: Vec<PathBuf> = see_dir(PathBuf::from("src"), cpp);

    let compiler = if cpp { "g++" } else { "gcc" };

    let status = Command::new(compiler)
        .args(files)
        .arg("-o")
        .arg("build/debug/debug.exe")
        .arg("-W")
        .arg("-Wall")
        .arg("-Werror")
        .arg("-Wextra")
        .status()
        .expect("Error while running compilation command.");

    if status.code() == Some(0) {
        println!("{}", "Compiled project successfully !".green())
    }
}
pub fn buildhard(cpp: bool) {
    if !Path::new("src").exists() {
        eprintln!("src/ folder not found. Make sure to be in a valid project");
        std::process::exit(36);
    }
    let files: Vec<PathBuf> = see_dir(PathBuf::from("src"), cpp);

    let compiler = if cpp { "g++" } else { "gcc" };

    let status = Command::new(compiler)
        .args(files)
        .arg("-o")
        .arg("build/release/release.exe")
        .arg("-W")
        .arg("-Wall")
        .arg("-Werror")
        .arg("-Wextra")
        .arg("-O3")
        .status()
        .expect("Error while running compilation command.");

    if status.code() == Some(0) {
        println!("{}", "Compiled project successfully !".green())
    }
}
pub fn buildcustom() {
    if Path::new("build.py").exists() {
        let content = lines_from_file("project.json").join("\n");
        let json: Value = match serde_json::from_str(&content) {
            Ok(j) => j,
            Err(_e) => {
                eprintln!("Failed to parse project.json");
                std::process::exit(66);
            }
        };

        if json["pyinterpreter"] == Value::Null {
            let ver = Command::new("python")
                .arg("--version")
                .output()
                .expect("Failed to get python version");
            let messagechars: Vec<char> =
                std::str::from_utf8(&ver.stdout).unwrap().chars().collect();

            if messagechars[7] < '3' && messagechars[9] < '5' {
                eprintln!("Python version has to be 3.5 or newer");
                std::process::exit(65);
            }
            Command::new("python")
                .arg("build.py")
                .status()
                .expect("Failed to run build script");
        } else {
            let pypath = match &json["pyinterpreter"] {
                Value::String(s) => s,
                _ => {
                    eprintln!("Pyinterpreter has to be a valid string");
                    std::process::exit(67);
                }
            };

            let ver = Command::new(pypath)
                .arg("--version")
                .output()
                .expect("Failed to get python version");
            let messagechars: Vec<char> =
                std::str::from_utf8(&ver.stdout).unwrap().chars().collect();

            if messagechars[7] < '3' && messagechars[9] < '5' {
                eprintln!("Python version has to be 3.5 or newer");
                std::process::exit(65);
            }
            Command::new(pypath)
                .arg("build.py")
                .status()
                .expect("Failed to run build script");
        }
    } else if Path::new("build.rb").exists() {
        let content = lines_from_file("project.json").join("\n");
        let json: Value = match serde_json::from_str(&content) {
            Ok(j) => j,
            Err(_e) => {
                eprintln!("Failed to parse project.json");
                std::process::exit(66);
            }
        };

        if json["rbinterpreter"] == Value::Null {
            let ver = Command::new("ruby")
                .arg("--version")
                .output()
                .expect("Failed to get ruby version");

            let messagechars: Vec<char> =
                std::str::from_utf8(&ver.stdout).unwrap().chars().collect();

            if messagechars[5] < '2' || (messagechars[5] < '2' && messagechars[7] < '3') {
                eprintln!("Ruby version has to be 2.3 or newer");
                std::process::exit(65);
            }
            Command::new("ruby")
                .arg("build.rb")
                .status()
                .expect("failed to run build script");
        } else {
            let rbpath = match &json["rbinterpreter"] {
                Value::String(s) => s,
                _ => {
                    eprintln!("rbinterpretrer has to be a valid string");
                    std::process::exit(67);
                }
            };
            let ver = Command::new(rbpath)
                .arg("--version")
                .output()
                .expect("Failed to get ruby version");

            let messagechars: Vec<char> =
                std::str::from_utf8(&ver.stdout).unwrap().chars().collect();

            if messagechars[5] < '2' || (messagechars[5] < '2' && messagechars[7] < '3') {
                eprintln!("Ruby version has to be 2.3 or newer");
                std::process::exit(65);
            }
            Command::new(rbpath)
                .arg("build.rb")
                .status()
                .expect("failed to run build script");
        }
    } else {
        eprintln!("Error: build script not found");
        std::process::exit(-65);
    }
}
