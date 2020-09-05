use std::env;
use std::io::{self, Write};
use std::path::Path;
#[allow(unused_imports)]
use std::process::exit;

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

#[cfg(test)]
mod test {
    use super::*;
    use lines_from_file::lines_from_file;

    #[test]
    fn creation_successful() -> std::io::Result<()> {
        create("project")?;

        let expected =
            String::from("{\n    \"name\" : \"project\",\n    \"version\" : \"0.1.0\",\n    \"standard\" : \"C99\",\n    \"author\" : \"Example <example@example.com>\",\n    \"dependencies\" : [  ]\n}");

        assert_eq!(
            Path::new(
                "C:\\Users\\Wafelack\\Documents\\Développement\\Rust\\wng\\project\\project.json"
            )
            .exists(),
            true
        );
        assert_eq!(
            lines_from_file(
                "C:\\Users\\Wafelack\\Documents\\Développement\\Rust\\wng\\project\\project.json"
            )
            .join("\n"),
            expected
        );
        Ok(())
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
