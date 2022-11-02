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

fn get_num_by_length(n: usize, lookup: Vec<&str>) -> &str
{
    for number in lookup
    {
        let length = number.to_string().chars().into_iter().count();
        if length == n { return number }
    }

    return "This should not be here";
}

fn get_overlap(n1: &str, n2: &str) -> usize
{
    return n1.chars().filter(|&c| n2.contains(c)).count();
}

fn solve(lines: Vec<String>) 
{
    let mut total: u64 = 0; 
    for line in lines
    {
        let parts = line.clone().split("|").map(|d| d.to_string()).collect::<Vec<_>>();

        let lookup = parts[0].split(" ").filter(|d| *d != "").collect::<Vec<_>>();
        let digits = parts[1].split(" ").filter(|d| *d != "").collect::<Vec<_>>();

        let one = get_num_by_length(2, lookup.clone());
        let four = get_num_by_length(4, lookup.clone());

        let mut decoded: Vec<&str> = vec![];

        for digit in digits
        {
            match digit.to_string().chars().into_iter().count() 
            {
                2 => decoded.push("1"),
                4 => decoded.push("4"),
                3 => decoded.push("7"),
                7 => decoded.push("8"),
                5 => {
                    if      get_overlap(digit, one)  == 2 { decoded.push("3") }
                    else if get_overlap(digit, four) == 2 { decoded.push("2") }
                    else                                  { decoded.push("5") }
                },
                6 => {
                    if      get_overlap(digit, four) == 4 { decoded.push("9") }
                    else if get_overlap(digit, one)  == 2 { decoded.push("0") }
                    else                                  { decoded.push("6") }
                },
                _ => panic!("We should not get here")
            }
        }
        total += decoded.join("").parse::<u64>().unwrap();
    }
    println!("Total: {}", total);
}

fn main() {
    let file: String = "input".to_string();
    let lines: Vec<String> = get_file(file);

    solve(lines);
}
