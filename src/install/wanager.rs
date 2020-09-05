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

        Ok(())
    }
}
