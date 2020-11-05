use std::env;
use std::io::{self, Write};
use std::path::Path;
#[allow(unused_imports)]
use std::process::exit;
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

struct Version {
    os: String,
    main: u8,
    discriminator: u8,
    third: u8,
}
impl Version {
    fn display(&self) {
        println!("Wanager by Wafelack <contactme.wafelack@protonmail.ch>, Licensed under MPL-v2.0, Version {} - {}.{}.{}", self.os, self.main, self.discriminator, self.third);
    }
    fn new(main: u8, discriminator: u8, third: u8) -> Version {
        Version {
            os: "Linux".to_string(),
            main,
            discriminator,
            third,
        }
    }
    fn set_os(&mut self, os: &str) {
        self.os = os.to_string();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn creation() -> std::io::Result<()> {
        create("test")?;
        let dir = &env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        assert!(Path::new(dir).exists());
        assert!(Path::new(&format!("{}\\project.json", dir)).exists());
        assert!(Path::new(&format!("{}\\deps.dat", dir)).exists());
        assert!(Path::new(&format!("{}\\src\\main.c", dir)).exists());

        Ok(())
    }
    #[test]
    fn building() -> std::io::Result<()> {
        env::set_current_dir("test")?;
        build();
        assert!(Path::new(".\\build\\debug\\debug.exe").exists());
        Ok(())
    }
    #[test]
    fn running() -> std::io::Result<()> {
        env::set_current_dir("test")?;
        run(vec![])?;
        Ok(())
    }
    #[test]
    fn archiving() -> std::io::Result<()> {
        env::set_current_dir("test")?;
        archive();
        let dir = &env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        println!("{}", format!("{}\\project.tar.gz", dir).as_str());
        assert!(Path::new(format!("{}\\project.tar.gz", dir).as_str()).exists());
        Ok(())
    }
}

fn main() {
    let mut ver = Version::new(3, 1, 0);

    if cfg!(windows) {
        ver.set_os("Windows");
    } else {
        ver.set_os("*Nix")
    }

    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    if argc < 2 {
        println!("Usage: wanager <command> [OPTIONS]");
        std::process::exit(1);
    }

    let displayhelp = || {
        ver.display();
        println!("\n--help | -h : displays this message");
        println!("--version | -v : displays version info");
        println!("\narchive : creates an archive with you project files");
        println!("new <name> : creates a new wanager project");
        println!("reinit [--force | -f] : reinitializes the project");
        println!("header <name> : creates a header file with basic header stuff");
        println!("test : runs tests contained in tests/tests.c");
        println!("\nbuild [--release | --custom] : compiles the project");
        println!("run : runs the compiled project");
        println!("\ninstall <source>:<username>/<repo> : installs the content of lib/ folder of the repo");
    };

    match argv[1].as_str() {
        "--version" => ver.display(),
        "-v" => ver.display(),
        "--help" => {
            displayhelp();
        }
        "-h" => {
            displayhelp();
        }
        "archive" => {
            if !Path::new("src").exists() {
                std::process::exit(-1);
            }
            archive();
        }
        "new" => {
            if argc != 3 {
                return;
            }
            match create(&argv[2]) {
                Ok(()) => (),
                Err(_e) => println!("An error occured. Please retry later"),
            }
        }
        "check" => {
            if !Path::new("project.json").exists() || !Path::new("deps.dat").exists() {
                std::process::exit(-1);
            }
            build();
            removebinary();
        }
        "build" => {
            if !Path::new("project.json").exists() {
                std::process::exit(-1);
            }
            if argc == 2 {
                build();
            } else if argc == 3 && argv[2].as_str() == "--release" {
                buildhard();
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
            if !Path::new("project.json").exists() || !Path::new("deps.dat").exists() {
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
            if !Path::new("project.json").exists() || !Path::new("deps.dat").exists() {
                std::process::exit(-1);
            }
            if argc != 3 {
                return;
            }
            install(&argv[2]);
        }
        "test" => {
            if !Path::new("tests/tests.c").exists() {
                println!("Create file `tests/tests.c` before testing");
                std::process::exit(-2);
            }
            match test() {
                Ok(()) => (),
                Err(s) => println!("{}", s),
            }
        }
        _ => println!("Usage: wanager <command> [OPTIONS]"),
    }
}
