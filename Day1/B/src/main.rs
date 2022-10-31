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
    let mut previous : Vec<i16> = vec![lines[0], lines[1], lines[2]];

    let mut total: i16;
    let mut prev_total: i16;

    for (i,line) in lines[3..lines.len()].into_iter().enumerate()
    {

        prev_total    = previous.iter().sum();
        previous[i%3] = *line;
        total         = previous.iter().sum(); 

        if total > prev_total {
            increased += 1;
        }
        
    }

    println!("It increased {} times!", increased);
}

fn main() {
    let file: String = "input".to_string();
    let lines: Vec<i16> = get_file(file);

    solve(lines);
}
