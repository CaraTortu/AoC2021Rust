use std::iter::Zip;
use std::ops::Range;
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

fn range(a: i16, b: i16) -> Range<i16>
{
    if a<=b { return a..b+1; }
    return b..a+1;
}

fn range_in_order(mut x1: i16, x2: i16, mut y1: i16, y2: i16) -> Zip<std::vec::IntoIter<i16>, std::vec::IntoIter<i16>>
{
    let mut a: Vec<i16> = vec![];
    let mut b: Vec<i16> = vec![];

    if x1 <= x2 
    { 
        while x1<=x2 {
            a.push(x1);
            x1 += 1;
        }
    }
    else 
    {
        while x1>=x2 {
            a.push(x1);
            x1 -= 1;
        }
        
    }

    if y1 <= y2 
    { 
        while y1<=y2 {
            b.push(y1);
            y1 += 1;
        }
    }
    else 
    {
        while y1>=y2 {
            b.push(y1);
            y1 -= 1;
        }
        
    }

    return a.into_iter().zip(b);

}

fn solve(lines: Vec<String>) 
{
    let mut floor: Vec<Vec<i8>> = vec![vec![0; 1000]; 1000];

    let mut x1:  i16;
    let mut y1:  i16;
    let mut x2:  i16;
    let mut y2:  i16;

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
            for i in range(y1, y2)
            {
                floor[i as usize][x1 as usize] += 1;
            }
        }

        else if y1 == y2
        {
            for i in range(x1, x2)
            {
                floor[y1 as usize][i as usize] += 1;
            }
        }

        else
        {
            for it in range_in_order(x1, x2, y1, y2)
            {
                let (x, y) = it;
                floor[y as usize][x as usize] += 1;
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
