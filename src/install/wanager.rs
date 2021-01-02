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

fn dl_n_check(link: String, lib: &str) -> Result<(), String> {
    let cloning = Command::new("git")
        .arg("clone")
        .arg(link)
        .output()
        .expect("Failed to git clone");

    if cloning.status.code() == Some(128) {
        // 128 is the Not Found code for Git
        return Err("Error, repository not found".to_owned());
    }
    if !Path::new(&format!("{}", lib)).exists() {
        return Err("Error, failed to clone repo into a folder".to_owned());
    }
    if !Path::new(&format!("{}/lib", lib)).exists() {
        return Err("Error, please select repo with a valid format (https://github.com/wafelack/wng/blob/master/README.md#to-install-a-library if you don't know)".to_owned());
    }
    match fs_extra::dir::move_dir(
        &format!("{}/lib/", lib),
        "src",
        &fs_extra::dir::CopyOptions::new(),
    ) {
        Ok(_) => (),
        Err(e) => return Err(format!("{}", e)),
    }
    match std::fs::rename(&format!("src/lib"), &format!("src/{}", lib)) {
        Ok(_) => (),
        Err(e) => return Err(format!("{}", e)),
    }
    match std::fs::remove_dir_all(lib) {
        Ok(_) => return Ok(()),
        Err(e) => {
            if e.kind() == ErrorKind::PermissionDenied {
                return Err(format!(
                    "You don't have enough permissions to delete folder `{}`",
                    lib
                ));
            }
            return Err(format!("{}", e));
        }
    }
}

impl Wanager {
    pub fn install<'a>(&self, source: &Source) -> Result<(), String> {
        let splited: Vec<&str> = source.unwrap().split('/').collect();
        if splited.len() != 2 {
            return Err(format!("`{}` is not a valid repository !", source.unwrap()));
        }

        // preparing link for git clone
        match source {
            Source::GitHub(_repo) => {
                let link = format!("https://github.com/{}/{}/", splited[0], splited[1]);

                dl_n_check(link, splited[1])
            }
            Source::GitLab(_repo) => {
                let link = format!("https://gitlab.com/{}/{}/", splited[0], splited[1]);

                dl_n_check(link, splited[1])
            }
            Source::BitBucket(_repo) => {
                let link = format!("https://bitbucket.org/{}/{}/", splited[0], splited[1]);

                dl_n_check(link, splited[1])
            }
            _ => Ok(()),
        }
    }
}
