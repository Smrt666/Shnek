use std::fs::OpenOptions;
use std::fs::{self};
use std::io::Write; // brings `write!` and `writeln!`

pub struct Score {
    file: String,
    do_write: bool,
}

impl Score {
    pub fn new() -> Self {
        Self {
            file: String::from("assets/scores.txt"),
            do_write: true,
        }
    }

    pub fn write(&mut self, score: usize) {
        if self.do_write {
            let mut write_file = OpenOptions::new()
                //append ro write
                .create(true)
                .append(true)
                .open(self.file.clone())
                .expect("Failed to open file for appending");

            write!(write_file, "\n{}", score).expect("Failed to append");
            // write!(write_file, "\n{}", previous).expect("Failed to append");

            self.do_write = false;
        }
    }

    pub fn reset(&mut self) {
        self.do_write = true;
    }

    pub fn read(&self) -> String {
        let content = fs::read_to_string(self.file.clone()).unwrap_or_else(|_| String::new());
        let mut scores = content.split('\n').collect::<Vec<&str>>();
        scores.reverse();
        scores.join("\n")
    }
}
