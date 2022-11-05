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

fn clamp(i: usize, max: usize, more: bool) -> usize
{
    if more 
    {
        if i >= max { return max }
        return i+1
    }

    if i == 0   { return i }
    return i-1
}

fn solve(lines: Vec<String>) 
{
    let mut map: Vec<Vec<i8>> = vec![];
    let mut total_value: u32 = 0;

    for line in lines
    {
        let line = line.split("").filter(|d| *d != "").collect::<Vec<_>>();
        map.push(line.iter().map(|d| d.parse::<i8>().unwrap()).collect::<Vec<_>>())
    }

    for (i, row) in map.iter().enumerate()
    {
        for (j, col) in row.into_iter().enumerate()
        {

            let va: i8;
            let vb: i8;
            let vc: i8;
            let vd: i8;

            if i == 0 
            { 
                va = 10;
                vd = map[clamp(i, map.len()-1, true)][j];
            }
            else if i == map.len()-1
            { 
                vd = 10;
                va = map[clamp(i, map.len()-1, false)][j];
            }
            else 
            {
                va = map[clamp(i, map.len()-1, false)][j];
                vd = map[clamp(i, map.len()-1, true)][j];
            }

            if j == 0
            { 
                vb = 10;
                vc = map[i][clamp(j, row.len()-1, true)];
            }
            else if j == row.len()-1
            { 
                vc = 10;
                vb = map[i][clamp(j, row.len()-1, false)];
            }
            else 
            {
                vb = map[i][clamp(j, row.len()-1, false)];
                vc = map[i][clamp(j, row.len()-1, true)];
            }

            if *col < va && *col < vb &&
               *col < vc && *col < vd
            {
                total_value += *col as u32 +1;
            }
        }
    }

    println!("Total: {}", total_value);

}

fn main() {
    let file: String = "input".to_string();
    let lines: Vec<String> = get_file(file);

    solve(lines);
}
