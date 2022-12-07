use std::fs::{File, read_to_string};
use std::io::{self, BufRead};

pub fn input_string(day: u32) -> String {
    read_to_string(format!("src/day_{day}/input.txt")).unwrap()
}

pub fn input_lines(day: u32) -> Vec<String> {
    let file = File::open(format!("src/day_{day}/input.txt")).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}