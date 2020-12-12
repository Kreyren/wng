use lines_from_file::lines_from_file;
use serde_json::*;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::str;

mod build;
mod install;
mod project;

use build::build::{build, buildcustom, buildhard, removebinary};
use build::run::run;
use install::install::install;
use project::archive::archive;
use project::create::create;
use project::header::header;
use project::reinit::reinit;
use project::testing::test;

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn creation() -> std::io::Result<()> {
        let twd = env::current_dir()?;
        let wd = twd.as_path().to_str().unwrap();

        create("foo", false)?;

        if cfg!(windows) {
            assert!(Path::new(&format!("{}\\foo\\project.json", wd)).exists());
            assert!(Path::new(&format!("{}\\foo\\src", wd)).exists());
            assert!(Path::new(&format!("{}\\foo\\tests", wd)).exists());
            assert!(Path::new(&format!("{}\\foo\\build", wd)).exists());
            fs::remove_dir_all(&format!("{}\\foo", wd))?;
        } else {
            assert!(Path::new(&format!("{}/foo/project.json", wd)).exists());
        assert!(Path::new(&format!("{}/foo/src", wd)).exists());
        assert!(Path::new(&format!("{}/foo/tests", wd)).exists());
        assert!(Path::new(&format!("{}/foo/build", wd)).exists());
        fs::remove_dir_all(&format!("{}/foo", wd))?;
        }

        Ok(())
    }

    #[test]
    fn header_creation() -> std::io::Result<()> {
        header("foo")?;
        let content = fs::read_to_string("foo.h")?;
        assert_eq!(
            content.trim(),
            "#ifndef _FOO_H_\n#define _FOO_H_\n\n\n\n#endif /* _FOO_H_ */"
        );
        fs::remove_file("foo.h")?;
        Ok(())
    }
}

fn displayhelp() {
    println!(
        "Wanager by {} version {}\n",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_VERSION")
    );
    println!("usage: wng <options> [flags]");
    println!("\nOPTIONS:");
    println!("\tarchive       : creates an archive with your projects file");
    println!("\tnew <name>    : creates a new project");
    println!("\treinit        : reinitializes the project");
    println!("\theader <name> : creates a header file");
    println!("\ttest          : runs tests contained in tests/tests.c");
    println!("\tbuild         : compiles the project");
    println!("\trun           : runs the compiled project");
    println!(
        "\tinstall <source>:<username>/<repo> : installs the content of lib/ folder of the repo"
    );
    println!("\nOPTIONS:");
    println!("\t--help | -h    : displays help");
    println!("\t--version | -v : displays version info");
    println!("\t--force | -f   : force reinitialization");
    println!("\t--release      : builds the project in release mode (high optimization level)");
    println!("\t--custom       : builds the project following a custom script");
    println!("\t--cpp          : creates a new C++ project");
}

/// Tests if the project is a C++ project
///
/// It reads project.json content and checks the "standard" key
fn is_cpp() -> bool {
    let json: Value = match serde_json::from_str(&lines_from_file("project.json").join("\n")) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Failed to parse project.json");
            eprintln!("Debug info : {}", e);
            std::process::exit(67);
        }
    };
    let cpp = if let Value::String(s) = &json["standard"] {
        let cpp = if s.starts_with("C++") { true } else { false };
        cpp
    } else {
        false
    };
    cpp
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    if argc < 2 {
        displayhelp();
        std::process::exit(1);
    }

    match argv[1].as_str() {
        "--version" | "-v" => println!("{}", env!("CARGO_PKG_VERSION")),
        "--help" | "-h" => {
            displayhelp();
        }
        "archive" => {
            if !Path::new("src").exists() {
                std::process::exit(-1);
            }
            archive();
        }
        "new" => {
            if argc < 3 {
                return;
            }
            let cpp = if argc == 4 {
                &argv[3] == "--cpp"
            } else {
                false
            };
            match create(&argv[2], cpp) {
                Ok(()) => (),
                Err(_e) => println!("An error occured. Please retry later"),
            }
        }
        "check" => {
            if !Path::new("project.json").exists() {
                std::process::exit(-1);
            }

            build(is_cpp());
            removebinary();
        }
        "build" => {
            if !Path::new("project.json").exists() {
                std::process::exit(-1);
            }
            if argc == 2 {
                build(is_cpp());
            } else if argc == 3 && argv[2].as_str() == "--release" {
                buildhard(is_cpp());
            } else if argc == 3 && argv[2].as_str() == "--custom" {
                buildcustom();
            }
        }
        "run" => {
            let mut args: Vec<&str> = Vec::new();
            for i in 2..argc {
                args.push(&argv[i]);
            }
            let ret = run(args);
            match ret {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            }
        }
        "reinit" => {
            if !Path::new("project.json").exists() {
                std::process::exit(-1);
            }
            if argc == 3 && argv[2].as_str() == "--force" || argc == 3 && argv[2].as_str() == "-f" {
                match reinit() {
                    Ok(_) => (),
                    Err(_e) => println!("Error while reinitializing directory"),
                }
            } else {
                print!("Really want to reinit ? [y/N] : ");
                io::stdout().flush().unwrap();
                let mut answer = String::new();
                io::stdin()
                    .read_line(&mut answer)
                    .expect("Error while reading your choice. Please retry later");
                if answer.trim().to_uppercase().as_str() == "Y" {
                    match reinit() {
                        Ok(_) => (),
                        Err(e) => println!("Error while reinitializing directory : {}", e),
                    }
                } else {
                    println!("Reinitialisation aborted");
                }
            }
        }
        "header" => {
            if argc != 3 {
                return;
            }
            match header(&argv[2]) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            }
        }
        "install" => {
            if !Path::new("project.json").exists() {
                std::process::exit(-1);
            }
            if argc != 3 {
                return;
            }
            install(&argv[2]);
        }
        "test" => {
            if !Path::new("project.json").exists() {
                eprintln!("Not in a wanager project");
                std::process::exit(-1);
            }

            match test(is_cpp()) {
                Ok(()) => (),
                Err(s) => println!("{}", s),
            }
        }
        _ => displayhelp(),
    }
}
