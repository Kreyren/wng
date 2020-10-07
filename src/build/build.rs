use lines_from_file::lines_from_file;
use std::path::Path;
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
    if !Path::new("build.py").exists() {
        eprintln!("Build script not found");
        std::process::exit(64);
    }
    Command::new("python")
        .arg("build.py")
        .spawn()
        .expect("Failed to run build script");
}
