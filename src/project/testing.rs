use see_directory::see_dir;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

#[allow(unused_assignments)]
pub fn test<'a>() -> Result<(), &'a str> {
    if cfg!(windows) {
        if !Path::new("tests\\tests.c").exists() {
            return Err("tests/tests.c not found");
        }
    } else {
        if !Path::new("tests/tests.c").exists() {
            return Err("tests/tests.c not found");
        }
    }
    let mut list: Vec<PathBuf> = Vec::new();
    match see_dir(PathBuf::from("src"), &mut list, true) {
        Ok(_) => (),
        Err(_e) => return Err("Failed to read dir"),
    }
    let mut files = String::new();
    for l in list {
        if !l.to_str().unwrap().ends_with("main.c")
            && l.extension().unwrap_or(OsStr::new("none")) == "c"
        {
            files.push_str(l.to_str().unwrap());
            files.push_str(" ");
        }
    }

    let mut status = Command::new("echo")
        .arg("Started testing ...")
        .status()
        .unwrap();

    if files != String::new() {
        files.pop();
        if cfg!(windows) {
            status = Command::new("gcc")
                .arg("tests/tests.c")
                .args(files.replace("\\", "/").split(' '))
                .arg("-o")
                .arg("tests/tests.exe")
                .status()
                .expect("Failed to call gcc");
        } else {
            status = Command::new("gcc")
                .arg("tests/tests.c")
                .args(files.split(' '))
                .arg("-o")
                .arg("tests/tests.exe")
                .status()
                .expect("Failed to call gcc");
        }
    } else {
        status = Command::new("gcc")
            .arg("tests/tests.c")
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
        Command::new("rm")
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
