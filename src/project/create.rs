use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use std::env;
use std::path::Path;

fn mkdir(name: &str, errmess: &str, number: u8) {
    match fs::create_dir(name) {
        Ok(_) => (),
        Err(_e) => println!("{} - {}", number, errmess),
    }
}

pub fn create(name: &str) -> std::io::Result<()> {
    let errmess: &str = "Error in process. Please retry later";

    let mut src: String = name.into();
    let mut tests: String = name.clone().into();
    let mut build: String = name.clone().into();
    let mut release: String = build.clone();
    let mut debug: String = build.clone();
    let mut main: String = src.clone();
    let mut testfile: String = tests.clone();

    if cfg!(windows) {
        src.push_str("\\src");

        tests.push_str("\\tests");

        build.push_str("\\build");

        release.push_str("\\release");

        debug.push_str("\\debug");

        main.push_str("\\main.c");

        testfile.push_str("\\tests.c");
    } else if cfg!(linux) {
        src.push_str("/src");
        tests.push_str("/tests");
        build.push_str("/build");
        release.push_str("/release");
        debug.push_str("/debug");
        main.push_str("/main.c");
    }

    mkdir(name, errmess, 1);
    mkdir(&tests, errmess, 2);
    mkdir(&src, errmess, 3);
    mkdir(&build, errmess, 4);
    mkdir(&release, errmess, 5);
    mkdir(&debug, errmess, 6);

    let mut mf = File::create(main)?;
    mf.write_all(b"#include <stdio.h>\n")?;
    mf.write_all(b"#include <stdlib.h>\n")?;
    mf.write_all(b"int main(void) {\n")?;
    mf.write_all(b"    puts(\"Hello, World !\");\n")?;
    mf.write_all(b"    return EXIT_SUCCESS;\n")?;
    mf.write_all(b"}")?;

    let mut tf = File::create(testfile)?;
    tf.write_all(b"#include <stdio.h>\n")?;
    tf.write_all(b"#include <stdlib.h>\n")?;
    tf.write_all(b"int main(void) {\n")?;
    tf.write_all(b"    puts(\"Hello, World !\");\n")?;
    tf.write_all(b"    return EXIT_SUCCESS;\n")?;
    tf.write_all(b"}")?;

    let mut gitignore: String = name.clone().into();
    if cfg!(windows) {
        gitignore.push_str("\\.gitignore");
    } else if cfg!(linux) {
        gitignore.push_str("/.gitignore");
    }

    let mut locker = File::create(gitignore)?;
    locker.write_all(b"build/")?;

    let project = Path::new(name);
    match env::set_current_dir(&project) {
        Ok(_) => (),
        Err(_e) => println!("Failed to change directory"),
    }
    /* JSON content
    {
        "name" : $name,
        "version" : " 0.1.0",
        "standard" : "C99",
        "author" : "Example <example@example.com>"
    }
    */
    let mut json = File::create("project.json")?;
    json.write_all(b"{\n")?;
    json.write_all(format!("    \"name\" : \"{}\",\n", name).as_bytes())?;
    json.write_all(b"    \"version\" : \"0.1.0\",\n")?;
    json.write_all(b"    \"standard\" : \"C99\",\n")?;
    json.write_all(b"    \"author\" : \"Example <example@example.com>\"\n")?;
    json.write_all(b"}")?;

    let mut readme = File::create("README.md")?;
    readme.write_all(format!("# {}", name).as_bytes())?;

    Command::new("git")
        .arg("init")
        .status()
        .expect("Failed to create git repository");
    println!("Created new project in {}", name);

    Ok(())
}
