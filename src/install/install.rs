use crate::install::wanager::*;

fn identify(lib: &str) -> Source {
    let splited: Vec<&str> = lib.split(':').collect();

    match splited[0] {
        "github" => return Source::GitHub(splited[1]),
        "gitlab" => return Source::GitLab(splited[1]),
        "bitbucket" => return Source::BitBucket(splited[1]),
        &_ => return Source::Error("Invalid source"),
    }
}

pub fn install(lib: &str) {
    let w = Wanager;

    let source = match identify(lib) {
        Source::Error(e) => {
            println!("{}", e);
            std::process::exit(-1);
        }
        _ => identify(lib),
    };

    w.install(&source);
    println!("Library `{}` was succesfully installed !", source.unwrap());
}
