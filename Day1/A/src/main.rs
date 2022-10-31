use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_file(f: String) -> Vec<i16> 
{
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);

    // Prepare Vector that contains the lines
    let mut contents: Vec<i16> = vec![];

    // Read line by line
    for line in reader.lines()
    {
        let line = line.expect("Error when reading file").parse::<i16>();
        contents.push(line.unwrap());
    }

    return contents;
}

fn solve(lines: Vec<i16>) 
{
    let mut increased: i16 = 0;
    let mut previous : i16 = lines[0];

    for line in lines
    {

        if line > previous {
            increased += 1;
        }

        previous = line;
        
    }

    println!("It increased {} times!", increased);
}

fn main() {
    let file: String = "input".to_string();
    let lines: Vec<i16> = get_file(file);

    solve(lines);
}
