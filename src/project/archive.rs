use std::process::Command;

pub fn archive() {
    Command::new("tar").arg("-czf").arg("project.tar.gz").arg("src").status().expect("Failed to run tar");
}