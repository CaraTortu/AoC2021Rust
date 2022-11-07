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

fn count_gp(map: &mut Vec<Vec<i8>>, i: i16, j: i16, gp: &mut Vec<i16>)
{
    if j < 0 || i < 0 || j >= map.len() as i16 || i >= map[0].len() as i16 || 
       map[j as usize][i as usize] == 9 || map[j as usize][i as usize] == -1
    {
        return
    }

    let last_index = gp.len()-1;

    map[j as usize][i as usize] = -1;
    gp[last_index] += 1;
    count_gp(map, i+1, j, gp);
    count_gp(map, i-1, j, gp);
    count_gp(map, i, j+1, gp);
    count_gp(map, i, j-1, gp);
}

fn solve(lines: Vec<String>) 
{
    let mut map: Vec<Vec<i8>> = vec![];
    let mut gp: Vec<i16> = vec![];
    let mut fin: i32 = 1;

    for line in lines
    {
        let line = line.split("").filter(|d| *d != "").collect::<Vec<_>>();
        map.push(line.iter().map(|d| d.parse::<i8>().unwrap()).collect::<Vec<_>>());
    }

    for i in 0..map.len()
    {
        for j in 0..map[0].len()
        {
            gp.push(0);
            count_gp(&mut map, j as i16, i as i16, &mut gp)
        }
    }

    let mut gp = gp.iter().filter(|d| **d != 0).collect::<Vec<_>>();
    let gp_len = gp.len();
    gp.sort();

    for i in 0..3
    {
        fin *= *gp[gp_len-1-i] as i32;
    }

    println!("Total: {}", fin);

}

fn main() {
    let file: String = "input".to_string();
    let lines: Vec<String> = get_file(file);

    solve(lines);
}
