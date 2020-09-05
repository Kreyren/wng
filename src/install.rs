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
        &_ => return Source::Error(&format!("`{}` source is not valid !", splited[0])),
    }
}

#[allow(unused_variables)]
pub fn install(lib: &str) -> std::io::Result<()> {
    let w = Wanager;
    if !Path::new("project.json").exists() {
        return Err(Error::new(ErrorKind::Other, "Not in a wanager project"));
    }

    let source = match identify(lib) {
        Source::Error(e) => return Err(Error::new(ErrorKind::Other, e)),
        _ => identify(lib),
    };

    match w.install() {
        Ok(_) => (),
        Err(e) => println!("Library {} is already installed !", lib),
    }
    fs::write("deps.dat", lib.as_bytes())?;
    println!("Library `{}` was succesfully installed in project !", lib);

    Ok(())
}
