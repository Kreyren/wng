use std::fs;
use toml::Value;
use std::io::Write;

pub fn manually(path: Option<&str>, key: &str, value: &str) -> crate::Result<()> {
    let home_dir = dirs::home_dir().unwrap();
    let default = format!("{}/wng.config", home_dir.to_str().unwrap());
    let config_file = path.map(|x| x.to_owned())
    .unwrap_or(
        default
    );

    let mut tomlized = fs::read_to_string(&config_file)?.parse::<Value>().unwrap();
    tomlized.as_table_mut().unwrap().insert(key.to_owned(), Value::String(value.to_owned()));

    fs::remove_file(&config_file)?;

    let mut file = fs::File::create(&config_file)?;

    file.write_all(
        toml::to_string(&tomlized).unwrap().as_bytes()
    )?;

    Ok(())
}