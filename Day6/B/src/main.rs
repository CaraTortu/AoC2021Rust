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

fn solve(lines: &String, days: i16) 
{
   let mut fishes = [0u64; 9];

   for fish in lines.split(",")
   {
        fishes[fish.parse::<usize>().unwrap()] += 1;
   }

   for _ in 0..days
   {
        let to_grow = fishes[0];

        for j in 1..9
        {
            fishes[j-1] = fishes[j];
        }

        fishes[8] = to_grow;
        fishes[6] += to_grow
   }

   let mut total: u64 = 0;

   for i in fishes
   {
        total += i;
   }

   println!("There are {} fishes after {} days", total, days);

}

fn main() {
    let file: String = "input".to_string();
    let lines: Vec<String> = get_file(file);

    solve(lines.first().unwrap(), 256);
}
