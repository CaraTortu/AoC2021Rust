use std::fs::File;
use std::io::{BufRead, BufReader};

struct Submarine {
    pub x: i32,
    pub y: i32,
    pub aim: i32
}

fn get_file(f: String) -> Vec<String> 
{
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);

    // Prepare Vector that contains the lines
    let mut contents: Vec<String> = vec![];

    // Read line by line
    for line in reader.lines()
    {
        let line = line.expect("Error when reading file").parse::<String>();
        contents.push(line.unwrap());
    }

    return contents;
}

fn solve(lines: Vec<String>) 
{
    let mut s = Submarine{ x: 0, y: 0, aim: 0 };

    for line in lines
    {
        let line_split = line.split(" ").collect::<Vec<_>>();
        let d = line_split[1].parse::<i32>().unwrap();

        if      line_split[0] == "forward" { s.x += d; s.y += s.aim*d }
        else if line_split[0] == "down"    { s.aim += d; }
        else if line_split[0] == "up"      { s.aim -= d; }
    }

    println!("X: {}\nY: {}\nAim: {}\nTotal: {}", s.x, s.y, s.aim, s.x*s.y);
}

fn main() {
    let file: String = "input".to_string();
    let lines: Vec<String> = get_file(file);

    solve(lines);
}
