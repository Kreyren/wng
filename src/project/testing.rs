use std::path::{Path, PathBuf};
use std::process::Command;

/// Reads a dir recursively
/// 
/// Used to get all src/ files to add them to the compilation command
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

/// Compiles src/ (without main.rs) and tests/tests.c to run tests
/// 
#[allow(unused_assignments)]
pub fn test<'a>(cpp: bool) -> Result<(), &'a str> {
    if cfg!(windows) {
        if !cpp {
            if !Path::new("tests\\tests.c").exists() {
                return Err("tests/tests.c not found");
            }
        } else {
            if !Path::new("tests\\tests.cpp").exists() {
                return Err("tests/tests.cpp not found");
            }
        }
    } else {
        if !cpp {
            if !Path::new("tests/tests.c").exists() {
                return Err("tests/tests.c not found");
            }
        } else {
            if !Path::new("tests/tests.cpp").exists() {
                return Err("tests/tests.cpp not found");
            }
        }
    }
    let list = see_dir(PathBuf::from("src"), cpp);
    let mut files = String::new();

    for l in list {
        files.push_str(l.as_path().to_str().unwrap_or(""));
        files.push_str(" ");
    }

    let mut status = Command::new("echo")
        .arg("Started testing ...")
        .status()
        .unwrap();

        let (testfile, compiler) = if cpp {
            ("tests/tests.cpp", "g++")
        } else {
            ("tests/tests.c", "gcc")
        };

    if files != String::new() {
        files.pop();
        if cfg!(windows) {
            status = Command::new(compiler)
                .arg(testfile)
                .args(files.replace("\\", "/").split(' '))
                .arg("-o")
                .arg("tests/tests.exe")
                .status()
                .expect("Failed to call gcc");
        } else {
            status = Command::new(compiler)
                .arg(testfile)
                .args(files.split(' '))
                .arg("-o")
                .arg("tests/tests.exe")
                .status()
                .expect("Failed to call gcc");
        }
    } else {
        status = Command::new(compiler)
            .arg(testfile)
            .arg("-o")
            .arg("tests/tests.exe")
            .status()
            .expect("Failed to call gcc");
    }
    if status.code() != Some(0) {
        return Err("Compilation failed");
    }

    if cfg!(windows) {
        Command::new(".\\tests\\tests.exe")
            .status()
            .expect("Failed to run program");
        Command::new("del")
            .arg(".\\tests\\tests.exe")
            .spawn()
            .expect("Failed to delete program");
    } else {
        Command::new("./tests/tests.exe")
            .status()
            .expect("Failed to run program");
        Command::new("rm")
            .arg("./tests/tests.exe")
            .spawn()
            .expect("Failed to delete program");
    }
    Ok(())
}
