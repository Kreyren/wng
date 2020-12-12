use fs_extra;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;
use std::str;

pub struct Wanager;
pub enum Source<'a> {
    GitLab(&'a str),
    GitHub(&'a str),
    BitBucket(&'a str),
    Error(&'a str),
}
impl<'a> Source<'a> {
    pub fn unwrap(&self) -> &str {
        let val = match self {
            Source::GitHub(repo) => repo,
            Source::GitLab(repo) => repo,
            Source::BitBucket(repo) => repo,
            _ => "",
        };
        val
    }
}

fn dl_n_check(link: String, lib: &str) {
    let cloning = Command::new("git")
        .arg("clone")
        .arg(link)
        .output()
        .expect("Failed to git clone");

    if cloning.status.code() == Some(128) {
        // 128 is the Not Found code for Git
        println!("Error, repository not found");
        std::process::exit(-1);
    }
    if !Path::new(&format!("{}", lib)).exists() {
        println!("Error, failed to clone repo into a folder");
        std::process::exit(-2);
    }
    if !Path::new(&format!("{}/lib", lib)).exists() {
        println!("Error, please select repo with a valid format (https://github.com/wmanage/wng/blob/master/README.md#to-install-a-library if you don't know)");
        std::process::exit(-3);
    }
    match fs_extra::dir::move_dir(
        &format!("{}/lib/", lib),
        "src",
        &fs_extra::dir::CopyOptions::new(),
    ) {
        Ok(_) => (),
        Err(e) => println!("Failed to move dir : {}", e),
    }
    match std::fs::rename(&format!("src/lib"), &format!("src/{}", lib)) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
    match std::fs::remove_dir_all(lib) {
        Ok(_) => (),
        Err(e) => {
            if e.kind() == ErrorKind::PermissionDenied {
                eprintln!("You don't have enough permissions to delete folder");
                std::process::exit(96);
            }
            println!("{}", e);
            std::process::exit(-4);
        }
    }
}

impl Wanager {
    pub fn install<'a>(&self, source: &Source) {
        let splited: Vec<&str> = source.unwrap().split('/').collect();
        if splited.len() != 2 {
            println!("Not a valid repository");
            std::process::exit(-1);
        }

        // preparing link for git clone
        match source {
            Source::GitHub(_repo) => {
                let link = format!("https://github.com/{}/{}/", splited[0], splited[1]);

                dl_n_check(link, splited[1]);
            }
            Source::GitLab(_repo) => {
                let link = format!("https://gitlab.com/{}/{}/", splited[0], splited[1]);

                dl_n_check(link, splited[1]);
            }
            Source::BitBucket(_repo) => {
                let link = format!("https://bitbucket.org/{}/{}/", splited[0], splited[1]);

                dl_n_check(link, splited[1]);
            }
            _ => (),
        }
    }
}
