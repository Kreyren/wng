use lines_from_file::lines_from_file;
use see_directory::see_dir;
use serde_json::*;
use std::env;
use std::fs;
use std::path::{PathBuf, Path};
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

pub enum ErrType {
    RepoNotFound,
    NoFolder,
    MovingError,
    NameError,
    ReadingError,
    RenameError,
    VCSNotFound,
}

pub enum WngResult<'a> {
    Ok,
    Err(ErrType, &'a str),
}

impl Wanager {
    #[allow(unused_assignments)]
    pub fn install(&self, source: Source) -> WngResult {
        let splited: Vec<&str> = source.unwrap().split('/').collect();
        if splited.len() != 2 {
            return WngResult::Err(ErrType::NameError, "Not a valid repository");
        }
        // USE GITHUB API TO CURL REPO AND UNPACK IT WITH 7Z

        println!("'{}'", splited[0]);
        println!("'{}'", splited[1]);
        match source {
            Source::GitHub(_repo) => {

                let link = format!("https://api.github.com/repos/{}/{}/tarball/master", splited[0], splited[1]);
                let archive = format!("{}.tar.gz", splited[1]);

                Command::new("curl")
                    .arg(link)
                    .arg("-o")
                    .arg(archive.clone())
                    .status()
                    .expect("Failed to run command");

                let mut parsed: bool = false;

                let v: Value = match serde_json::from_str(&lines_from_file(&format!("{}.tar", splited[1])).join("\n")) {
                    Ok(()) => {
                        parsed = true;
                        serde_json::from_str(&lines_from_file(&format!("{}.tar", splited[1])).join("\n")).unwrap()
                    },
                    Err(_e) => {
                        parsed = false;
                        serde_json::from_str("{\"name\":\"did not worked\"}").unwrap()
                    }
                };
                println!("{}", parsed); // DEBUG

                if parsed {
                    if v["message"] != Value::Null && v["message"] == "\"Not Found\"" {
                        return WngResult::Err(ErrType::RepoNotFound, "Repo does not exists");
                    }
                }

                Command::new("tar")
                    .arg("-xzf").arg(archive.clone()).arg("-C").arg(format!("src/{}", splited[1])).status().expect("Failed to unpack");


                let mut inside: Vec<PathBuf> = vec![];
                match see_dir(PathBuf::from(format!("src/{}", splited[1])), &mut inside, true) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("{}",e);
                        return WngResult::Err(ErrType::ReadingError, "Failed to read directory l155");
                    }
                }

                let mut libexists: bool = false;

                for i in inside {
<<<<<<< HEAD
                    if i.to_str().unwrap() == "lib" && i.is_dir() && !libexists {
                        let lib: PathBuf = i;
=======
                    if i.to_str().unwrap() == "lib" && i.is_dir() {
                        let _lib: PathBuf = i;
>>>>>>> master
                        libexists = true;
                    }
                }
                match libexists {
                    false => (),
                    true => {
                        Command::new("mv").arg(format!("src/{}/lib/", splited[1])).arg("lib").spawn().expect("failed to move from src/<libname>/lib to lib/");
                    },
                }
                WngResult::Ok
            }
            _ => return WngResult::Err(ErrType::VCSNotFound, "Source does not exists"),
        }
    }
}
