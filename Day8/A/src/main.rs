use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

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
    let mut digits = [0u16; 9];

    for line in lines
    {
        let splitted = line.split("|").collect::<Vec<_>>();

        for digit in splitted[1].to_string().split(" ")
        {
            let length = digit.to_string().chars().count();

            if length == 2 { digits[1] += 1; }
            if length == 4 { digits[4] += 1; }
            if length == 3 { digits[7] += 1; }
            if length == 7 { digits[8] += 1; }
        }
    }

    let total: u16 = digits.iter().sum();

    println!("Sum of 1, 4, 7, 8: {}", total);
}

fn main() {
    let file: String = "input".to_string();
    let lines: Vec<String> = get_file(file);

    solve(lines);
}
