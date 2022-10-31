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
    let mut floor: Vec<Vec<i8>> = vec![vec![0; 1000]; 1000];

    let mut x1:  i16;
    let mut y1:  i16;
    let mut x2:  i16;
    let mut y2:  i16;
    let mut x3:  i16;
    let mut y3:  i16;
    let mut x4:  i16;
    let mut y4:  i16;

    let mut a: String;
    let mut b: String;

    let mut two_overlap: i16 = 0;

    for line in lines
    {
        let mut splitted = line.split("->").collect::<Vec<_>>();

        a = splitted[0].to_string();
        b = splitted[1].to_string();

        splitted = a.split(",").collect::<Vec<_>>();

        x1 = splitted[0].parse::<i16>().unwrap();
        y1 = splitted[1].replace(" ", "").parse::< i16>().unwrap();

        splitted = b.split(",").collect::<Vec<_>>();

        x2 = splitted[0].replace(" ", "").parse::< i16>().unwrap();
        y2 = splitted[1].parse::<i16>().unwrap();

        if x1 == x2 
        {
            if y2 > y1 { y3 = y1; y4 = y2+1; }
            else       { y3 = y2; y4 = y1+1; }

            for i in y3..y4
            {
                floor[i as usize][x1 as usize] += 1;
            }
        }

        else if y1 == y2
        {
            if x2 > x1 { x3 = x1; x4 = x2+1; } 
            else       { x3 = x2; x4 = x1+1; }

            for i in x3..x4
            {
                floor[y1 as usize][i as usize] += 1;
            }
        }
    }

    for row in &floor
    {
        for col in row
        {
            if *col >= 2 {
                two_overlap += 1;
            }
        }
    }

    println!("Points overlapping: {}", two_overlap);

}

fn main() {
    let file: String = "input".to_string();
    let lines: Vec<String> = get_file(file);

    solve(lines);
}
