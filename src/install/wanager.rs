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
        self.unwrap()
    }
}

impl Wanager {
    pub fn install(&self, source: Source) -> std::io::Result<()> {
        match std::fs::create_dir(&path) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
