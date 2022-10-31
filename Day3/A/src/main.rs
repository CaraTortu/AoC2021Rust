use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_file(f: String) -> Vec<String> 
{
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);

    // Prepare Vector that contains the lines
    let mut contents: Vec<String> = vec![];

    // Read line by line
    for line in reader.lines()
    {
        let line = line.unwrap();
        contents.push(line);
    }

    return contents;
}

fn solve(lines: Vec<String>) 
{
    let mut zeros: i32 = 0;
    let mut ones: i32 = 0;

    let mut gamma: String = "".to_string();
    let mut epsilon: String = "".to_string();

    for i in 0..lines[0].chars().count() {
        for line in &lines
        {
            if line.chars().nth(i).unwrap() == '0' { zeros += 1 }
            else { ones  += 1 }
        }

        if zeros > ones
        {
            gamma   += "0";
            epsilon += "1";
        }

        else if ones > zeros
        {
            gamma   += "1";
            epsilon += "0";
        }

        ones = 0;
        zeros = 0;
    }

    let gamma_n   = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_n = isize::from_str_radix(&epsilon, 2).unwrap();

    println!("Gamma is {}\nEpsilon is {}\nTotal power {}", gamma_n, epsilon_n, gamma_n*epsilon_n);

}

fn main() {
    let file: String = "input".to_string();
    let lines: Vec<String> = get_file(file);

    solve(lines);
}
