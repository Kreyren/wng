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
    let mut src = String::new();
    let mut tests = String::new();
    let mut build = String::new();
    let mut release = String::new();
    let mut debug = String::new();
    let mut main = String::new();
    let mut testfile = String::new();

    if cfg!(windows) {
        src = name.into();
        src.push_str("\\src");
        tests = name.clone().into();
        tests.push_str("\\tests");
        build = name.clone().into();
        build.push_str("\\build");
        release = build.clone();
        release.push_str("\\release");
        debug = build.clone();
        debug.push_str("\\debug");
        main = src.clone();
        main.push_str("\\main.c");
        testfile = tests.clone();

        testfile.push_str("\\tests.c");
    } else if cfg!(linux) {
        src = name.into();
        src.push_str("/src");
        tests = name.clone().into();
        tests.push_str("/tests");
        build = name.clone().into();
        build.push_str("/build");
        release = build.clone();
        release.push_str("/release");
        debug = build.clone();
        debug.push_str("/debug");
        main = src.clone();
        main.push_str("/main.c");
        testfile = tests.clone();
        testfile.push_str("/tests.c");
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
