use colored::*;
use lines_from_file::lines_from_file;
use serde_json::Value;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn see_dir(dir: PathBuf, cpp: bool) -> Result<Vec<PathBuf>, String> {
    let mut list: Vec<PathBuf> = Vec::new();
    for entry in match std::fs::read_dir(dir.clone()) {
        Ok(e) => e,
        Err(e) => return Err(format!("{}", e)),
    } {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => return Err(format!("{}", e)),
        };
        if entry.path().is_dir() {
            let sub: Vec<PathBuf> = see_dir(entry.path(), cpp)?;
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
    Ok(list)
}

pub fn removebinary() -> Result<(), String> {
    match std::fs::remove_file("build/debug/debug.exe") {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e)),
    }
}

fn generic_build(cpp: bool, with_opt: bool) -> Result<(), String> {
    if !Path::new("src").exists() {
        return Err("src/ folder not found. Make sure to be in a valid project".to_owned());
    }
    let rawfiles: Vec<PathBuf> = see_dir(PathBuf::from("src"), cpp)?;
    let mut files: Vec<&PathBuf> = vec![];

    if Path::new(".wngignore").exists() {
        let to_ignore = lines_from_file(".wngignore");

        'master: for i in 0..rawfiles.len() {
            let curfile = rawfiles[i].to_str().unwrap();
            for ign in &to_ignore {
                if !curfile.starts_with(ign) {
                    continue;
                } else {
                    continue 'master;
                }
            }
            files.push(&rawfiles[i]);
        }
    }

    let compiler = if cpp { "g++" } else { "gcc" };

    let status = if with_opt {
        match Command::new(compiler)
            .args(&files)
            .arg("-o")
            .arg("build/debug/debug.exe")
            .arg("-W")
            .arg("-Wall")
            .arg("-Werror")
            .arg("-Wextra")
            .status()
        {
            Ok(s) => s,
            Err(e) => return Err(format!("{}", e)),
        }
    } else {
        match Command::new(compiler)
            .args(&files)
            .arg("-o")
            .arg("build/debug/debug.exe")
            .arg("-W")
            .arg("-Wall")
            .arg("-Werror")
            .arg("-Wextra")
            .arg("-O3")
            .status()
        {
            Ok(s) => s,
            Err(e) => return Err(format!("{}", e)),
        }
    };

    if status.code() == Some(0) {
        println!("{}", "Compiled project successfully !".green());
        return Ok(());
    } else {
        return Err(format!("{}", "Error while compiling project !".red()));
    }
}

pub fn build(cpp: bool) -> Result<(), String> {
    generic_build(cpp, false)
}
pub fn build_optimized(cpp: bool) -> Result<(), String> {
    generic_build(cpp, true)
}
pub fn buildcustom() -> Result<(), String> {
    if Path::new("build.py").exists() {
        let content = lines_from_file("project.json").join("\n");
        let json: Value = match serde_json::from_str(&content) {
            Ok(j) => j,
            Err(e) => return Err(format!("{}", e)),
        };

        if json["pyinterpreter"] == Value::Null {
            let ver = match Command::new("python").arg("--version").output() {
                Ok(o) => o,
                Err(e) => return Err(format!("{}", e)),
            };
            let messagechars: Vec<char> = match std::str::from_utf8(&ver.stdout) {
                Ok(u) => u,
                Err(e) => return Err(format!("{}", e)),
            }
            .chars()
            .collect();

            if messagechars[7] < '3' && messagechars[9] < '5' {
                return Err("Python version has to be 3.5 or newer".to_owned());
            }
            match Command::new("python").arg("build.py").status() {
                Ok(_) => {}
                Err(e) => return Err(format!("{}", e)),
            }
        } else {
            let pypath = match &json["pyinterpreter"] {
                Value::String(s) => s,
                _ => return Err("Pyinterpreter has to be a valid string".to_owned()),
            };

            let ver = match Command::new(pypath).arg("--version").output() {
                Ok(o) => o,
                Err(e) => return Err(format!("{}", e)),
            };
            let messagechars: Vec<char> = match std::str::from_utf8(&ver.stdout) {
                Ok(u) => u,
                Err(e) => return Err(format!("{}", e)),
            }
            .chars()
            .collect();

            if messagechars[7] < '3' && messagechars[9] < '5' {
                return Err("Python version has to be 3.5 or newer".to_owned());
            }
            match Command::new(pypath).arg("build.py").status() {
                Ok(_) => {}
                Err(e) => return Err(format!("{}", e)),
            }
        }
    } else if Path::new("build.rb").exists() {
        let content = lines_from_file("project.json").join("\n");
        let json: Value = match serde_json::from_str(&content) {
            Ok(j) => j,
            Err(e) => return Err(format!("{}", e)),
        };

        if json["rbinterpreter"] == Value::Null {
            let ver = match Command::new("ruby").arg("--version").output() {
                Ok(o) => o,
                Err(e) => return Err(format!("{}", e)),
            };

            let messagechars: Vec<char> =
                std::str::from_utf8(&ver.stdout).unwrap().chars().collect();

            if messagechars[5] < '2' || (messagechars[5] < '2' && messagechars[7] < '3') {
                return Err("Ruby version has to be 2.3 or newer".to_owned());
            }
            match Command::new("ruby").arg("build.rb").status() {
                Ok(_) => {}
                Err(e) => return Err(format!("{}", e)),
            }
        } else {
            let rbpath = match &json["rbinterpreter"] {
                Value::String(s) => s,
                _ => {
                    return Err("rbinterpretrer has to be a valid string".to_owned());
                }
            };
            let ver = match Command::new(rbpath).arg("--version").output() {
                Ok(o) => o,
                Err(e) => return Err(format!("{}", e)),
            };

            let messagechars: Vec<char> =
                std::str::from_utf8(&ver.stdout).unwrap().chars().collect();

            if messagechars[5] < '2' || (messagechars[5] < '2' && messagechars[7] < '3') {
                return Err("Ruby version has to be 2.3 or newer".to_owned());
            }
            match Command::new(rbpath).arg("build.rb").status() {
                Ok(_) => {}
                Err(e) => return Err(format!("{}", e)),
            }
        }
    } else {
        return Err("Build script not found !".to_owned());
    }
    Ok(())
}
