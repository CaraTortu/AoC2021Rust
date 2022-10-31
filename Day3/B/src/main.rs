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

fn get_o2(mut lines: Vec<String>) -> isize
{
    let mut zeros: i32 = 0;
    let mut ones: i32 = 0;

    let mut o2: String = "".to_string();

    for i in 0..lines[0].chars().count() {
        for line in &lines
        {
            if line.chars().nth(i).unwrap() == '0' { zeros += 1 }
            else { ones  += 1 }
        }

        if zeros > ones
        {
            o2  += "0";
        } 

        else 
        {
            o2  += "1";
        }

        lines = lines.into_iter().filter(|l| l.starts_with(&o2)).collect::<Vec<_>>();

        if lines.len() == 1 {
            o2 = lines.into_iter().nth(0).unwrap();
            break;
        }

        ones = 0;
        zeros = 0;
    }

    let o2_n   = isize::from_str_radix(&o2, 2).unwrap();

    println!("O2 is {}", o2_n);
    return o2_n;
}

fn get_co2(mut lines: Vec<String>) -> isize
{
    let mut zeros: i32 = 0;
    let mut ones: i32 = 0;

    let mut co2: String = "".to_string();

    for i in 0..lines[0].chars().count() {
        for line in &lines
        {
            if line.chars().nth(i).unwrap() == '0' { zeros += 1 }
            else { ones  += 1 }
        }

        if zeros <= ones
        {
            co2  += "0";
        } 

        else 
        {
            co2  += "1";
        }

        lines = lines.into_iter().filter(|l| l.starts_with(&co2)).collect::<Vec<_>>();

        if lines.len() == 1 {
            co2 = lines.into_iter().nth(0).unwrap();
            break;
        }

        ones = 0;
        zeros = 0;
    }

    let o2_n   = isize::from_str_radix(&co2, 2).unwrap();

    println!("CO2 is {}", o2_n);
    return o2_n;
}

fn solve(lines: Vec<String>) 
{
    let o2 = get_o2(lines.clone());
    let co2 = get_co2(lines.clone());

    println!("Total value {}", o2*co2);
}

fn main() {
    let file: String = "input".to_string();
    let lines: Vec<String> = get_file(file);

    solve(lines);
}
