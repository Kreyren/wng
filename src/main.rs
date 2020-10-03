use std::env;
use std::io::{self, Write};
use std::path::Path;
#[allow(unused_imports)]
use std::process::exit;
use std::str;

mod build;
mod create;
mod header;
mod install;
mod query;
mod reinit;
mod run;
mod testing;

use build::{build, buildcustom, buildhard};
use create::create;
use header::header;
use install::install;
use query::query;
use reinit::reinit;
use run::run;
use testing::test;

struct Version {
    os: String,
    main: u8,
    discriminator: u8,
    third: u8,
}
impl Version {
    fn display(&self) {
        println!("Wanager by Wafelack <contactme.wafelack@protonmail.ch>, Licensed under GPL-v3.0, Version {} - {}.{}.{}", self.os, self.main, self.discriminator, self.third);
    }
}

#[cfg(test)]
mod test {}

fn main() {
    let ver = Version {
        os: String::from("Windows"),
        main: 2,
        discriminator: 11,
        third: 2,
    };
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    if argc < 2 {
        println!("Usage: wanager <command> [OPTIONS]");
        std::process::exit(1);
    }
    match argv[1].as_str() {
        "--version" => ver.display(),
        "new" => {
            if argc != 3 {
                return;
            }
            match create(&argv[2]) {
                Ok(()) => (),
                Err(_e) => println!("An error occured. Please retry later"),
            }
        }
        "build" => {
            if !Path::new("project.json").exists() {
                std::process::exit(-1);
            }
            if argc == 3 && argv[2].as_str() == "--release" {
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
            if !Path::new("project.json").exists() {
                std::process::exit(-1);
            }
            if argc == 3 && argv[2].as_str() == "--force" {
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
            if argc != 3 {
                return;
            }
            install(&argv[2]);
        }
        "query" => {
            if argc != 3 {
                return;
            }
            query(&argv[2]);
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
