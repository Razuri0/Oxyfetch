use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use sysinfo::System;

#[derive(Clone)]
pub enum EXPRESSIONS {
    NAME,
    RELEASE_TYPE,
    KERNEL,
    ASCII,
}

//-----------------------------

pub struct Module {
    bullets: Vec<EXPRESSIONS>,
    paths: Vec<PathBuf>,
}

//-----------------------------

impl Module {
    pub fn new(bullets: Vec<EXPRESSIONS>) -> Self {
        let mut paths = Vec::new();
        paths.push(Path::new("ascii.txt").to_path_buf());

        Module { bullets, paths }
    }

    pub fn format_bullets(&mut self) -> Vec<String> {
        let mut return_value = Vec::new();

        let bullets = self.bullets.clone(); // avoid immutable + mutable borrow
        for bullet in bullets {
            match bullet {
                EXPRESSIONS::NAME => {
                    return_value.push(self.name());
                }
                EXPRESSIONS::RELEASE_TYPE => {
                    return_value.push(self.release_type());
                }
                EXPRESSIONS::KERNEL => {
                    return_value.push(self.kernel());
                }
                EXPRESSIONS::ASCII => {
                    return_value.append(&mut self.ascii());
                }
            }
        }

        return_value
    }

    //-------------------------

    fn name(&self) -> String {
        System::name().unwrap_or_else(|| "Unknown".to_string())
    }

    fn release_type(&self) -> String {
        System::os_version().unwrap_or_else(|| "Unknown".to_string())
    }

    fn kernel(&self) -> String {
        System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
    }

    //-------------------------

    fn ascii(&mut self) -> Vec<String> {
        let mut return_value = Vec::new();

        if let Some(path) = self.paths.pop() {
            if let Ok(lines) = Self::read_lines(&path) {
                for line in lines.flatten() {
                    return_value.push(line);
                }
            }
        }

        return_value
    }

    //-------------------------

    fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
