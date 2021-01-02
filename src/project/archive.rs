use std::fs::File;
use tar::Builder;

pub fn archive() -> Result<(), String> {
    let file = match File::create("project.tar") {
        Ok(f) => f,
        Err(e) => return Err(format!("{}", e)),
    };
    let mut builder = Builder::new(file); // Creates a new archive builder to put the files in
    match builder.append_dir_all("src", "src") {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    }
    Ok(())
}
