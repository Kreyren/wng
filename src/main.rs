use lines_from_file::lines_from_file;
use serde_json::Value;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::str;

mod build;
mod install;
mod project;

use build::build::{build, build_optimized, buildcustom, removebinary};
use build::run::run;
use install::install::install;
use project::archive::archive;
use project::create::create;
use project::header::header;
use project::reinit::reinit;
use project::testing::test;

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[cfg(unix)]
    #[test]
    fn creation() -> Result<(), String> {
        let twd = match env::current_dir() {
            Ok(d) => d,
            Err(e) => return Err(format!("{}", e)),
        };
        let wd = twd.as_path().to_str().unwrap();

        create("foo", false)?;

        if cfg!(windows) {
            assert!(Path::new(&format!("{}\\foo\\project.json", wd)).exists());
            assert!(Path::new(&format!("{}\\foo\\src", wd)).exists());
            assert!(Path::new(&format!("{}\\foo\\tests", wd)).exists());
            assert!(Path::new(&format!("{}\\foo\\build", wd)).exists());
            match fs::remove_dir_all(&format!("{}\\foo", wd)) {
                Ok(()) => {}
                Err(e) => return Err(format!("{}", e)),
            }
        } else {
            assert!(Path::new(&format!("{}/foo/project.json", wd)).exists());
            assert!(Path::new(&format!("{}/foo/src", wd)).exists());
            assert!(Path::new(&format!("{}/foo/tests", wd)).exists());
            assert!(Path::new(&format!("{}/foo/build", wd)).exists());
            match fs::remove_dir_all(&format!("{}/foo", wd)) {
                Ok(()) => {}
                Err(e) => return Err(format!("{}", e)),
            }
        }

        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn header_creation() -> Result<(), String> {
        header("foo")?;
        let content = match fs::read_to_string("foo.h") {
            Ok(s) => s,
            Err(e) => return Err(format!("{}", e)),
        };
        assert_eq!(
            content.trim(),
            "#ifndef _FOO_H_\n#define _FOO_H_\n\n\n\n#endif /* _FOO_H_ */"
        );
        match fs::remove_file("foo.h") {
            Ok(()) => {}
            Err(e) => return Err(format!("{}", e)),
        }
        Ok(())
    }
}

fn displayhelp() {
    println!(
        "Wanager by {} version {}\n",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_VERSION")
    );
    println!("usage: wng <options> [flags]");
    println!("\nOPTIONS:");
    println!("\tarchive       : creates an archive with your projects file");
    println!("\tnew <name>    : creates a new project");
    println!("\treinit        : reinitializes the project");
    println!("\theader <name> : creates a header file");
    println!("\ttest          : runs tests contained in tests/tests.c");
    println!("\tbuild         : compiles the project");
    println!("\trun           : runs the compiled project");
    println!(
        "\tinstall <source>:<username>/<repo> : installs the content of lib/ folder of the repo"
    );
    println!("\nOPTIONS:");
    println!("\t--help | -h    : displays help");
    println!("\t--version | -v : displays version info");
    println!("\t--force | -f   : force reinitialization");
    println!("\t--release      : builds the project in release mode (high optimization level)");
    println!("\t--custom       : builds the project following a custom script");
    println!("\t--cpp          : creates a new C++ project");
}

/// Tests if the project is a C++ project
///
/// It reads project.json content and checks the "standard" key
fn is_cpp() -> Result<bool, String> {
    let json: Value = match serde_json::from_str(&lines_from_file("project.json").join("\n")) {
        Ok(j) => j,
        Err(e) => return Err(format!("{}", e)),
    };
    let cpp = if let Value::String(s) = &json["standard"] {
        let cpp = if s.starts_with("C++") { true } else { false };
        cpp
    } else {
        false
    };
    Ok(cpp)
}

fn init() -> Result<(), String> {
    Ok(())
}

fn main() -> Result<(), String> {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    if argc < 2 {
        displayhelp();
        std::process::exit(1);
    }

    match argv[1].as_str() {
        "--version" | "-v" => println!("{}", env!("CARGO_PKG_VERSION")),
        "--help" | "-h" => {
            displayhelp();
        }
        "archive" => {
            if !Path::new("src").exists() {
                std::process::exit(-1);
            }
            archive()?;
        }
        "new" => {
            if argc < 3 {
                return Err("Fatal: Invalid arguments".to_owned());
            }
            let cpp = if argc == 4 {
                &argv[3] == "--cpp"
            } else {
                false
            };
            create(&argv[2], cpp)?;
        }
        "check" => {
            if !Path::new("project.json").exists() {
                return Err("Missing project.json".to_owned());
            }
            if argc == 2 {
                build(is_cpp()?)?;
                removebinary()?;
            } else if argc == 3 && argv[2].as_str() == "--custom" {
                buildcustom()?;
                eprintln!("Due to custom build, binary produced by your script cannot be removed automatically.");
            }
        }
        "build" => {
            if !Path::new("project.json").exists() {
                return Err("Missing project.json".to_owned());
            }
            if argc == 2 {
                build(is_cpp()?)?;
            } else if argc == 3 && argv[2].as_str() == "--release" {
                build_optimized(is_cpp()?)?;
            } else if argc == 3 && argv[2].as_str() == "--custom" {
                buildcustom()?;
            }
        }
        "run" => {
            let mut args: Vec<&str> = Vec::new();
            for i in 2..argc {
                args.push(&argv[i]);
            }
            run(args)?;
        }
        "reinit" => {
            if !Path::new("project.json").exists() {
                return Err("Missing project.json".to_owned());
            }
            if argc == 3 && argv[2].as_str() == "--force" || argc == 3 && argv[2].as_str() == "-f" {
                match reinit() {
                    Ok(_) => (),
                    Err(e) => return Err(format!("{}", e)),
                }
            } else {
                print!("Really want to reinit ? [y/N] : ");
                io::stdout().flush().unwrap();
                let mut answer = String::new();
                io::stdin()
                    .read_line(&mut answer)
                    .expect("Error while reading your choice. Please retry later");
                if answer.trim().to_uppercase().as_str() == "Y" {
                    reinit()?;
                } else {
                    return Err("Reinitialisation aborted".to_owned());
                }
            }
        }
        "header" => {
            if argc != 3 {
                return Err(format!("Fatal: Invalid arguments"));
            }
            header(&argv[2])?;
        }
        "install" => {
            if !Path::new("project.json").exists() {
                std::process::exit(-1);
            }
            if argc != 3 {
                return Err(format!("Fatal: Invalid arguments "));
            }
            install(&argv[2])?;
        }
        "test" => {
            if !Path::new("project.json").exists() {
                eprintln!("Not in a wanager project");
                std::process::exit(-1);
            }

            test(is_cpp()?)?;
        }
        _ => displayhelp(),
    }
    Ok(())
}
