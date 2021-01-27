use std::path::Path;
use std::{fs, fs::File};

pub fn reinit(path: Option<&str>) -> crate::Result<()> {
    let home_dir = dirs::home_dir().unwrap();
    let default = format!("{}/wng.config", home_dir.to_str().unwrap());
    let config_file = path.map(|x| x.to_owned()).unwrap_or(
        default
    );

    if Path::new(&config_file).exists() {
        fs::remove_file(&config_file)?;
    }

    File::create(&config_file)?;

    Ok(())
}