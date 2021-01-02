use std::fs::File;
use std::io::Write;
use std::process::Command;

use std::env;
use std::path::Path;

use crate::project::reinit::mkdir;

#[allow(unused_assignments)]
pub fn create(name: &str, cpp: bool, username: &str, email: &str) -> Result<(), String> {
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
        if cpp {
            main.push_str("\\main.cpp");
            testfile = tests.clone();

            testfile.push_str("\\tests.cpp");
        } else {
            main.push_str("\\main.c");
            testfile = tests.clone();

            testfile.push_str("\\tests.c");
        }
    } else {
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
        if cpp {
            main.push_str("/main.cpp");
            testfile = tests.clone();

            testfile.push_str("/tests.cpp");
        } else {
            main.push_str("/main.c");
            testfile = tests.clone();

            testfile.push_str("/tests.c");
        }
    }

    mkdir(name)?;
    mkdir(&tests)?;
    mkdir(&src)?;
    mkdir(&build)?;
    mkdir(&release)?;
    mkdir(&debug)?;

    if !cpp {
        let mut mf = match File::create(main) {
            Ok(f) => f,
            Err(e) => return Err(format!("{}", e)),
        };
        match mf.write_all(b"#include <stdio.h>\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match mf.write_all(b"#include <stdlib.h>\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match mf.write_all(b"int main(void) {\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match mf.write_all(b"    puts(\"Hello, World !\");\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match mf.write_all(b"    return EXIT_SUCCESS;\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match mf.write_all(b"}") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }

        let mut tf = match File::create(testfile) {
            Ok(f) => f,
            Err(e) => return Err(format!("{}", e)),
        };
        match tf.write_all(b"#include <stdio.h>\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match tf.write_all(b"#include <stdlib.h>\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match tf.write_all(b"int main(void) {\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match tf.write_all(b"    puts(\"Hello, World !\");\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match tf.write_all(b"    return EXIT_SUCCESS;\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match tf.write_all(b"}") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
    } else {
        let mut mf = match File::create(main) {
            Ok(f) => f,
            Err(e) => return Err(format!("{}", e)),
        };
        match mf.write_all(b"#include <iostream>\n\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match mf.write_all(b"int main() {\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match mf.write_all(b"    std::cout << \"Hello, World! \" << std::endl;\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match mf.write_all(b"    return 0;\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match mf.write_all(b"}") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }

        let mut tf = match File::create(testfile) {
            Ok(f) => f,
            Err(e) => return Err(format!("{}", e)),
        };
        match tf.write_all(b"#include <iostream>\n\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match tf.write_all(b"int main() {\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match tf.write_all(b"    std::cout << \"Hello, World! \" << std::endl;\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match tf.write_all(b"    return 0;\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        match tf.write_all(b"}") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
    }

    let mut gitignore: String = name.clone().into();
    if cfg!(windows) {
        gitignore.push_str("\\.gitignore");
    } else {
        gitignore.push_str("/.gitignore");
    }

    let mut locker = match File::create(gitignore) {
        Ok(f) => f,
        Err(e) => return Err(format!("{}", e)),
    };
    match locker.write_all(b"build/") {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    };

    let project = Path::new(name);
    match env::set_current_dir(&project) {
        Ok(_) => (),
        Err(e) => return Err(format!("{}", e)),
    }
    /* JSON content
    {
        "name" : $name,
        "version" : " 0.1.0",
        "standard" : "C99",
        "author" : "Example <example@example.com>"
    }
    */
    let mut json = match File::create("project.json") {
        Ok(f) => f,
        Err(e) => return Err(format!("{}", e)),
    };
    match json.write_all(b"{\n") {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    }
    match json.write_all(format!("    \"name\" : \"{}\",\n", name).as_bytes()) {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    }
    match json.write_all(b"    \"version\" : \"0.1.0\",\n") {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    }
    if !cpp {
        match json.write_all(b"    \"standard\" : \"C99\",\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
    } else {
        match json.write_all(b"    \"standard\" : \"C++14\",\n") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
    }
    match json.write_all(format!("    \"author\" : \"{} <{}>\"\n", username, email).as_bytes()) {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    }
    match json.write_all(b"}") {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    }

    let mut readme = match File::create("README.md") {
        Ok(f) => f,
        Err(e) => return Err(format!("{}", e)),
    };
    match readme.write_all(format!("# {}", name).as_bytes()) {
        Ok(()) => {}
        Err(e) => return Err(format!("{}", e)),
    }

    match Command::new("git").arg("init").status() {
        Ok(_) => {}
        Err(e) => return Err(format!("{}", e)),
    }
    println!("Created new project in {}", name);

    Ok(())
}
