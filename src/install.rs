mod wanager;
use std::io::{Error, ErrorKind};
use wanager::Wanager;
use std::path::Path;
use std::fs;

#[allow(unused_variables)]
pub fn install(lib: &str) -> std::io::Result<()> {
    let w = Wanager;
    if !w.exists(lib) {
        return Err(Error::new(ErrorKind::NotFound, "Library does not exists !"));
    }
    let links = w.get_link(lib);
    if !Path::new("lock.wmg").exists() && !Path::new("deps.dat").exists() {
        return Err(Error::new(ErrorKind::Other, "Not in a wanager project"));
    }
    match w.install(lib, links) {
        Ok(_) => (),
        Err(e) => println!("Library {} is already installed !", lib),
        }
    fs::write("deps.dat", lib.as_bytes())?;
    println!("Library `{}` was succesfully installed in project !", lib);


    Ok(())
}