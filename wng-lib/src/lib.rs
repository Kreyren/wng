mod tests;
mod config;
mod create;
mod errors;
mod deps;
pub use errors::*;

pub fn get_config_file(path: Option<&str>) -> String {
    let home_dir = dirs::home_dir().unwrap();
    let default = format!("{}/wng.config", home_dir.to_str().unwrap());
    path.map(|x| x.to_owned())
    .unwrap_or(
        default
    )
}