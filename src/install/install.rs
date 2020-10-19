use crate::install::wanager::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::ErrorKind;

fn identify(lib: &str) -> Source {
    let splited: Vec<&str> = lib.split(':').collect();

    match splited[0] {
        "github" => return Source::GitHub(splited[1]),
        "gitlab" => return Source::GitLab(splited[1]),
        "bitbucket" => return Source::BitBucket(splited[1]),
        &_ => return Source::Error("Invalid source"),
    }
}

#[allow(unused_variables)]
pub fn install(lib: &str) {
    let w = Wanager;
    if !Path::new("project.json").exists() {}

    let source = match identify(lib) {
        Source::Error(e) => {
            println!("{}", e);
            std::process::exit(-1);
        }
        _ => identify(lib),
    };

    w.install(source.clone());
    println!("Library `{}` was succesfully installed !", source.unwrap());
    let mut deps = match File::open("deps.dat") {
        Ok(f) => f,
        Err(_e) => {
            eprintln!("Failed to open deps.dat");
            std::process::exit(-5);
        }
    };
    let splitedsource: Vec<&str> = source.unwrap().split('/').collect();
    match deps.write_all(splitedsource[1].as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            if e.kind() == ErrorKind::PermissionDenied {
                eprintln!("You don't have enough permissions to write in file deps.dat");
                std::process::exit(96);
            }
            eprintln!("Failed to write in deps.dat");
            std::process::exit(-6);
        }
    }
}
