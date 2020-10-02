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
    pub fn clone(&self) -> Source {
        match self {
            Source::GitLab(repo) => return Source::GitLab(repo),
            Source::GitHub(repo) => return Source::GitHub(repo),
            Source::BitBucket(repo) => return Source::BitBucket(repo),
            Source::Error(e) => return Source::Error(e),
        }
    }
}

impl Wanager {
    pub fn install<'a>(&self, source: Source) {
        let splited: Vec<&str> = source.unwrap().split('/').collect();
        if splited.len() != 2 {
            println!("Not a valid repository");
            std::process::exit(-1);
        }
        match source {
            Source::GitHub(_repo) => {
                let link = format!("https://github.com/{}/{}/", splited[0], splited[1]);
                let cloning = Command::new("git")
                    .arg("clone")
                    .arg(link)
                    .output()
                    .expect("Failed to git clone");

                if cloning.status.code() == Some(128) {
                    println!("Error, repository not found");
                    std::process::exit(-1);
                }
                if !Path::new(&format!("{}", splited[1])).exists() {
                    println!("Error, failed to clone repo into a folder");
                    std::process::exit(-2);
                }
                if !Path::new(&format!("{}/lib", splited[1])).exists() {
                    println!("Error, please select repo with a valid format (https://github.com/wmanage/wng/blob/master/README.md#to-install-a-library if you don't know)");
                    std::process::exit(-3);
                }
            }
            _ => (),
        }
    }
}
