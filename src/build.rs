use serde_json::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;

fn lines_from_file(filename: impl AsRef<Path>) -> std::io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

pub fn build() {
    let lines: Vec<String> = lines_from_file("deps.dat").expect("Failed to get lines from file");
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
    let lines: Vec<String> = lines_from_file("deps.dat").expect("Failed to get lines from file");
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
    let data = match File::open("project.json") {
        Ok(f) => f,
        Err(_e) => {
            eprintln!("Failed to read data in project.json");
            std::process::exit(-3);
        }
    };
    let dat: Value = match serde_json::from_str(/* ADD A READING OF PROJECT.JSON*/) {
        Ok(v) => v,
        Err(_e) => {
            eprintln!("Failed to parse json");
            std::process::exit(-2);
        }
    };
}
