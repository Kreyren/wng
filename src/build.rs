use lines_from_file::lines_from_file;
use serde_json::*;
use std::process::Command;

pub fn build() {
    let lines: Vec<String> = lines_from_file("deps.dat");
    let mut files: Vec<String> = Vec::new();
    for i in 0..lines.len() {
        files.push(format!("src\\{}\\*.c", lines[i]));
    }

    Command::new("gcc")
        .arg("src/*.c")
        .args(files)
        .arg("-o")
        .arg("build/release/release.exe")
        .arg("-W")
        .arg("-Wall")
        .arg("-Werror")
        .arg("-Wextra")
        .spawn()
        .expect("Error while running compilation command.");
}
pub fn buildhard() {
    let lines: Vec<String> = lines_from_file("deps.dat");
    let mut files: Vec<String> = Vec::new();
    for i in 0..lines.len() {
        files.push(format!("src\\{}\\*.c", lines[i]));
    }

    Command::new("gcc")
        .arg("src/*.c")
        .args(files)
        .arg("-o")
        .arg("build/release/release.exe")
        .spawn()
        .expect("Error while running compilation command.");
}
pub fn buildcustom() {
    let dat: Value = match serde_json::from_str(&lines_from_file("project.json").join("\n")) {
        Ok(v) => v,
        Err(_e) => {
            eprintln!("Failed to parse json");
            std::process::exit(-2);
        }
    };
    if dat["build"] == Value::Null {
        eprintln!("No custom build profile found in project.json. Please add the field \"build\" with your build command in project.json");
        std::process::exit(-3);
    }
    let fullcommand: &String = match &dat["build"] {
        Value::String(s) => s,
        _ => {
            eprintln!("Build profile has to be a string !");
            std::process::exit(2);
        }
    };
    let splitedcommand: Vec<&str> = fullcommand.as_str().split(' ').collect();
    let program = splitedcommand[0];
    let mut args: Vec<&str> = vec![];
    for i in 1..splitedcommand.len() {
        args.push(splitedcommand[i]);
    }
    Command::new(program)
        .args(&args)
        .status()
        .expect("Error while running compilation commands");
}
