use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
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


fn main() {
    let ver = env!("CARGO_PKG_VERSION");

    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    if argc < 2 {
        println!("Usage: wng <command> [OPTIONS]");
        std::process::exit(1);
    }

    let displayhelp = || {
        println!("{}", ver);
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
        "--version" => println!("{}", ver),
        "-v" => println!("{}", ver),
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
        _ => println!("Usage: wng <command> [OPTIONS]"),
    }
}
