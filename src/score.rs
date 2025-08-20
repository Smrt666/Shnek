use std::fs::{self, read};
use std::fs::OpenOptions;
use std::io::Write; // brings `write!` and `writeln!`


pub struct Score{
    file : String,
    do_write: bool,
}


impl Score {
    pub fn new() -> Self{
        Self { 
            file: String::from("assets/scores.txt"),
            do_write: true,
            }
    }


    pub fn write(&mut self, previous: String, score: usize) {

        if self.do_write == true {
            let mut write_file = OpenOptions::new()
                .write(true)    //append ro write
                .create(true)
                .open(self.file.clone())
                .expect("Failed to open file for appending");

            write!(write_file, "{}", score).expect("Failed to append");
            write!(write_file, "\n{}", previous).expect("Failed to append");

            self.do_write = false;
        }
    }

    pub fn reset(&mut self) {
        self.do_write = true;
    }

    pub fn read(&self) -> String {
         return fs::read_to_string(self.file.clone())
                .unwrap_or_else(|_| String::new())
    }
}

