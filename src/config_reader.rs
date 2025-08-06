use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Config {
    tiles: Vec<TileConfig>,
}

#[derive(Debug, Deserialize)]
struct TileConfig {
    line: u16,
    row: u16,
    width: u16,
    height: u16,
    #[serde(rename = "type")]
    tile_modules: Option<Vec<Vec<String>>>,
    ascii_art: Option<String>, // for path as String to .txt for ASCII
}

fn read_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn config_reader() {
    match read_config("config.toml") {
        Ok(config) => {
            println!("Loaded config: {:?}", config);
        }
        Err(e) => eprintln!("Error reading config: {}", e),
    }
}