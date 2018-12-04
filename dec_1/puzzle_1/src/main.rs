use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("./input.txt").unwrap();
    let mut reader = BufReader::new(&f);
    let mut line = String::new();
    let mut result = 0;
    // note: the below could (should?) use reader.lines instead but...
    while 0 < reader.read_line(&mut line).unwrap()
    {
        match &line[0..1]
        {
            "+" => { result += &line[1..(line.len()-1)].parse::<i32>().unwrap() },
            _ => { result += &line[..(line.len()-1)].parse::<i32>().unwrap()},
        }
        line = String::new()
    }
    println!("Got {}", result);
}
