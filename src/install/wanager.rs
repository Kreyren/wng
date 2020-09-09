use lines_from_file::lines_from_file;
use serde_json::*;
use std::fs::rename;
use std::io::{Error, ErrorKind};
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

pub enum ErrType {
    RepoNotFound,
    FileNotFound,
    NoFolder,
    CurlError,
    NameError,
    CreationError,
    ReadingError,
    RenameError,
    VCSNotFound,
}

pub enum WngResult<'a> {
    Ok,
    Err(ErrType, &'a str),
}

impl Wanager {
    pub fn install(&self, source: Source) -> WngResult {
        let splited: Vec<&str> = source.unwrap().split('/').collect();
        if splited.len() != 2 {
            return WngResult::Err(ErrType::NameError, "Not a valid repository");
        }
        // USE GITHUB API TO CURL REPO AND UNPACK IT WITH 7Z

        match source {
            Source::GitHub(_repo) => {
                Command::new("curl")
                    .arg(&format!(
                        "https://api.github.com/repos/{}/{}/zipball/master",
                        splited[0], splited[1]
                    ))
                    .arg("-o")
                    .arg(&format!("{}.tar", splited[1]))
                    .output()
                    .expect("Failed to run command");

                let v: Value = match serde_json::from_str(
                    &lines_from_file(&format!("{}.tar", splited[1])).join("\n"),
                ) {
                    Ok(()) => serde_json::from_str(
                        &lines_from_file(&format!("{}.tar", splited[1])).join("\n"),
                    )
                    .unwrap(),
                    Err(_e) => {
                        return WngResult::Err(ErrType::ReadingError, "Failed to parse tarball")
                    }
                };

                if v["message"] != Value::Null && v["message"] == "\"Not Found\"" {
                    return WngResult::Err(ErrType::RepoNotFound, "Repo does not exists");
                }

                Command::new("tar")
                    .arg("-xvf")
                    .arg(&format!("{}.tar", splited[1]));

                let path = Path::new(".");
                for entry in path.read_dir().expect("Failed to read dir") {
                    if let Ok(entry) = entry {
                        if entry
                            .path()
                            .starts_with(&format!("{}-{}", splited[0], splited[1]))
                        {
                            match rename(entry.path(), &format!("{}-{}", splited[0], splited[1])) {
                                Ok(()) => break,
                                Err(_e) => {
                                    return WngResult::Err(
                                        ErrType::RenameError,
                                        "Failed to rename folder",
                                    );
                                }
                            }
                        }
                    } else {
                        return WngResult::Err(ErrType::RenameError, "Failed to rename folder");
                    }
                }

                // TODO : TRY TO FIND LIB & MOVE IT IN SRC/

                WngResult::Ok
            }
            _ => return WngResult::Err(ErrType::VCSNotFound, "Source does not exists"),
        }
    }
}
