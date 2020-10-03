mod wanager;
use std::path::Path;
use wanager::Wanager;

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
