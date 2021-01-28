use std::fs;
use toml::Value;
use std::io::Write;

pub fn manually(path: Option<&str>, key: &str, value: &str) -> crate::Result<()> {
    let config_file = crate::get_config_file(path);

    println!("{}", config_file);

    let mut tomlized = fs::read_to_string(&config_file)?.parse::<Value>().unwrap();
    tomlized.as_table_mut().unwrap().insert(key.to_owned(), Value::String(value.to_owned()));
    let mut file = fs::File::create(&config_file)?;

    file.write_all(
        toml::to_string(&tomlized).unwrap().as_bytes()
    )?;

    Ok(())
}