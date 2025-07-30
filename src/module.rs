use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum EXPRESSIONS {
    NAME,
    OS,
    KERNEL,
    ASCII,
}

//-----------------------------

struct module {
    bullets: Vec<EXPRESSIONS>,
    paths: Vec<std::path::PathBuf>,
}

//-----------------------------

impl module {

    pub fn format_bullets(&self) {
        let mut return_value = Vec::new();

        match self.bullets {
            NAME -> {
                return_value.push(self.name())
            }
            OS -> {
                return_value.push(self.os())
            }
            KERNEL -> {
                return_value.push(self.kernel())
            }
            ASCII -> {
                self.ascii(&mut return_value)
            }
        }

        return_value
    }

    //-------------------------

    // Returns the name of the system
    fn name() -> String {
        System::name().unwrap_or("Unknown".to_string())
    }

    //-------------------------

    // Returns the name of the os
    fn os() -> String {
        System::os().unwrap_or("Unknown".to_string())
    }

    //-------------------------

    // Return the kernel version
    fn kernel() -> String {
        System::kernel_version().unwrap_or("Unknown".to_string())
    }

    //-------------------------

    // Returns the content of a txt file line by line
    fn ascii(&return_value: &mut Vec<String>) {
        let path = self.paths.pop();
        if let Ok(lines) = read_lines(Path) {
            for line in lines.map_while(Result::ok) {
                return_value.push(line)
            }
        }
    }

    //-------------------------

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
        }
    }

}