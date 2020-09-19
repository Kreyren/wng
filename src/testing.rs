use std::path::{Path, PathBuf};
use std::process::Command;
use see_directory::see_dir;

pub fn test<'a>() -> Result<(), &'a str> {
    if !Path::new("tests\\tests.c").exists() {
        return Err("tests/tests.c not found");
    }
    let mut list: Vec<PathBuf> = Vec::new();
    match see_dir(PathBuf::from("src"), &mut list, true) {
        Ok(_) => (),
        Err(e) => return Err("Failed to read dir"),
    }
    let mut files = String::new();
    for l in list {
        if !l.to_str().unwrap().ends_with("main.c") && l.extension().unwrap() == "c" {
            files.push_str(l.to_str().unwrap());
            files.push_str(" ");
        }
    }


    if files != String::new() {
        files.pop();
        Command::new("gcc").arg("tests/tests.c").args(files.replace("\\", "/").split(' ')).arg("-o").arg("tests/tests.exe").status().expect("Failed to call gcc");
    } else {
        Command::new("gcc").arg("tests/tests.c").arg("-o").arg("tests/tests.exe").status().expect("Failed to call gcc");
    }
    if !Path::new("tests\\tests.exe").exists() {
        return Err("Compilation failed");
    }
    Command::new(".\\tests\\tests.exe").status().expect("Failed to run program");
    Command::new("rm").arg(".\\tests\\tests.exe").spawn().expect("Failed to delete program");
    Ok(())
}