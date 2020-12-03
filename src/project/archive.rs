use std::fs::File;
use tar::Builder;

pub fn archive() {
    let file = match File::create("project.tar") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create file project.tar");
            eprintln!("Debug info : {}", e);
            std::process::exit(66);
        }
    };
    let mut builder = Builder::new(file);
    match builder.append_dir_all("src", "src") {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Failed to add `src/` to archive");
            eprintln!("Debug info : {}", e);
            std::process::exit(67);
        }
    }
}
