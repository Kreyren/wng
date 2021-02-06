use crate::error;
use crate::errors::*;
use std::io::Write;
use std::path::Path;
use std::{fs, fs::File};

pub fn add_dep(dependencie: &str, messages: bool) -> crate::Result<()> {
    if !Path::new("project.toml").exists() {
        return Err(error!("Project.toml not found in the current directory."));
    }

    let mut tomlized: toml::Value = toml::from_str(&fs::read_to_string("project.toml")?)?;

    let deps = match tomlized["project"]["dependencies"].as_array_mut() {
        Some(array) => array,
        None => return Err(error!("Cannot find field `dependencies` in `project.toml`")),
    };

    let mut already = false;
    for dep in deps.clone() {
        if let toml::Value::String(s) = dep {
            if s == dependencie {
                already = true;
                break;
            }
        }
    }

    if !already {
        deps.push(toml::Value::String(dependencie.to_owned()));
    } else {
        if messages {
            println!("Dependency `{}` already is in `project.toml`.", dependencie);
        }
        return Ok(());
    }

    let mut file = File::create("./project.toml")?;

    file.write_all(toml::to_string(&tomlized)?.as_bytes())?;

    if messages {
        println!("[+] Successfully added dependency `{}`", dependencie);
    }

    Ok(())
}

pub fn remove_dep(dependencie: &str, messages: bool) -> crate::Result<()> {
    if !Path::new("project.toml").exists() {
        return Err(error!("Project.toml not found in the current directory."));
    }

    let mut tomlized: toml::Value = toml::from_str(&fs::read_to_string("project.toml")?)?;

    let deps = match tomlized["project"]["dependencies"].as_array_mut() {
        Some(array) => array,
        None => return Err(error!("Cannot find field `dependencies` in `project.toml`")),
    };

    for i in 0..deps.len() {
        if let toml::Value::String(s) = &deps[i] {
            if s == dependencie {
                deps.remove(i);
                break;
            }
        }
    }

    let mut file = File::create("./project.toml")?;

    file.write_all(toml::to_string(&tomlized)?.as_bytes())?;

    if messages {
        println!("[+] Successfully added dependency `{}`", dependencie);
    }

    Ok(())
}
