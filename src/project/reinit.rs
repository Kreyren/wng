use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn mkdir(name: &str) -> Result<(), String> {
    match fs::create_dir(name) {
        Ok(_) => Ok(()),
        Err(e) => return Err(format!("{}", e)),
    }
}

pub fn reinit() -> Result<(), String> {
    if !Path::new("project.json").exists() {
        return Err("Not in a wanager project".to_owned());
    }
    match fs::remove_dir_all("src") {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    };
    match fs::remove_dir_all("build") {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    };

    mkdir("src")?;
    mkdir("build")?;
    mkdir("build/release")?;
    mkdir("build/debug")?;

    let mut mf = match File::create("src/main.c") {
        Ok(f) => f,
        Err(e) => return Err(format!("{}", e)),
    };
    match mf.write_all(b"#include <stdio.h>\n#include <stdlib.h>\n\nint main(void) {\n\tputs(\"Hello, World !\");\n\treturn EXIT_SUCCESS;\n}") {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    }

    println!("Project reinitialized successfully !");
    Ok(())
}
