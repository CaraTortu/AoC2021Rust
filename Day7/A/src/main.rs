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

fn solve(lines: &String) 
{
   let mut positions = [0i64; 2000];

   for pos in lines.split(",")
   {
        positions[pos.parse::<usize>().unwrap()] += 1;
   }

   let mut min_res: i64 = -1;

   for x in 0..2000
   {
        let mut total_cost: i64 = 0;

        for (pos, amount) in positions.into_iter().enumerate()
        {
            total_cost += (pos as i64-x).abs() * amount;
        }

        if total_cost < min_res || min_res == -1
        {
            min_res = total_cost;
        }
   }

   println!("Minimum fuel {}", min_res);

}

fn main() {
    let file: String = "input".to_string();
    let lines: Vec<String> = get_file(file);

    solve(lines.first().unwrap());
}
