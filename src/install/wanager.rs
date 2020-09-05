use std::process::Command;
use std::str;

pub struct Wanager;
pub enum Source<'a> {
    GitLab(&'a str),
    GitHub(&'a str),
    BitBucket(&'a str),
    Error(&'a str),
}

impl Wanager {
    pub fn install(&self, lib_name: &str, source: Source) -> std::io::Result<()> {
        let path: String = format!("src\\{}", lib_name);
        match std::fs::create_dir(&path) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
