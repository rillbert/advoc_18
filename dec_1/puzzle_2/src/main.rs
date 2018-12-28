use std::fs;

fn line_to_i32(line: &str) -> i32
{
    match &line[0..1]
    {
        "+" => line[1..].parse::<i32>().unwrap(),
        _ => line.parse::<i32>().unwrap(),
    }
}

fn main() {
    println!("Puzzle_2, day 1");
    // use a hashmap to 'bin' the frequencies, the first time
    // a bin has more than one value, it is a duplication

    // read in the file to a buffer
    let f = fs::File.open("input.txt");
    let mut reader = BufReader.new(f);

    // loop until we have two hits for the same bin
    while true
    {
        let mut lines_iter = reader.lines().map(|line| line.unwrap());

    }


}
