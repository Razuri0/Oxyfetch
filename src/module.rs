use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path};
use sysinfo::System;

use config_reader::{read_config, Config, TileConfig};

#[derive(Clone)]
pub enum EXPRESSIONS {
    NAME,
    RELEASE_CYCLE,
    KERNEL,
    ASCII,
}

pub struct Bullet {
    pub module: EXPRESSIONS,
    pub value: String,
}

//-----------------------------

pub struct Tile {
    data: String,
    line: u16,
    row: u16,
}

//-----------------------------

impl Tile {
    
    pub fn new(config: TileConfig) {

        let mut tile = Tile {
            data: String::new(),
            line: config.line,
            row: config.row,
        };

        for lines in config.tile_modules.unwrap_or_default() {
            for rows in lines {
                match rows.as_str() {
                    "name" => tile.data.push_str(&format!("{}", tile.name())),
                    "release_cycle" => tile.data.push_str(&format!("{}", tile.release_cycle())),
                    "kernel" => tile.data.push_str(&format!("{}", tile.kernel())),
                    "ascii" => tile.data.push_str(&format!("{}", tile.ascii(&config))),
                    _ => eprintln!("{}", rows),
                }
            }
            tile.data.push('\n');
        }

    }

    //-------------------------

    fn name(&self) -> String {
        System::name().unwrap_or_else(|| "Unknown".to_string())
    }

    fn release_cycle(&self) -> String {
        System::os_version().unwrap_or_else(|| "Unknown".to_string())
    }

    fn kernel(&self) -> String {
        System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
    }

    //-------------------------

    fn ascii(&self, tile: &TileConfig) -> String {
        if let Some(ascii_art) = &tile.ascii_art {
            if let Ok(lines) = Self::read_lines(ascii_art) {
                lines.filter_map(Result::ok).collect::<Vec<String>>().join("\n")
            } else {
                "Error reading ASCII art".to_string()
            }
        } else {
            "No ASCII art provided".to_string()
        }
    }

    //-------------------------

    fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
