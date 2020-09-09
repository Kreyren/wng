use lines_from_file::lines_from_file;
use std::env;
use std::io::{self, Write};
use std::path::Path;
#[allow(unused_imports)]
use std::process::exit;
use std::process::Command;
use std::str;

mod build;
mod create;
mod header;
mod install;
mod query;
mod reinit;
mod run;

use build::{build, buildhard};
use create::create;
use header::header;
use install::install;
use query::query;
use reinit::reinit;
use run::run;

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

use serde_json::{Result, Value};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn not_found() -> Result<()> {
        let curling = Command::new("curl")
            .arg("https://api.github.com/repos/wafelack/dict/tarball/master")
            .arg("-o")
            .arg("test.tar")
            .output()
            .expect("Failed to run command");
        println!("{}", str::from_utf8(&curling.stderr).unwrap());
        let v: Value = serde_json::from_str(&lines_from_file("test.tar").join("\n"))?;
        if v["message"] != Value::Null {
            println!("{}", v["message"]);
        } else {
            println!("It's ok")
        }

        Ok(())

        /*
            {
                "message": "Not Found",
                "documentation_url": "https://docs.github.com/rest/reference/repos#download-a-repository-archive"
            }
            Vérifier que le .zip n'est pas égal à ça ^ pour voir si le repeo existe puis si il existe, le déziper, renommer le dossier
        (faut le trouver avec startswith) puis copier le dossier lib (verifier qu'il existe oc) dans src/ et le renommer par le nom
        du repo

            */
    }
}

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
            } else {
                buildhard();
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
            match install(&argv[2]) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            }
        }
        "query" => {
            if argc != 3 {
                return;
            }
            query(&argv[2]);
        }
        &_ => println!("Usage: wanager <command> [OPTIONS]"),
    }
}
