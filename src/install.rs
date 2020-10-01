mod wanager;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
use wanager::*;

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
}
