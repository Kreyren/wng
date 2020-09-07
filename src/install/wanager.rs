use std::io::{Error, ErrorKind};
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
    pub fn install(&self, source: Source) -> std::io::Result<()> {
        let splited: Vec<&str> = source.unwrap().split('/').collect();
        if splited.len() != 2 {
            return Err(Error::new(ErrorKind::Other, "Not a valid repository"));
        }

        match std::fs::create_dir(splited[1]) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        // USE GITHUB API TO CURL REPO AND UNPACK IT WITH 7Z

        let curling = Command::new("curl")
            .arg(&format!(
                "https://api.github.com/repos/{}/{}/zipball/master",
                splited[0], splited[1]
            ))
            .arg("-o")
            .arg(splited[1])
            .output()
            .expect("Failed to run command");
        println!("{:?}", &curling.stdout);

        Ok(())
    }
}
