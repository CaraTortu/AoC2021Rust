use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;
use std::process::exit;

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

fn get_chosen(lines: Vec<String>) -> Vec<i16>
{
    let chosen = lines[0].split(",").collect::<Vec<_>>();
    let mut f: Vec<i16> = vec![];

    for c in chosen
    {
        f.push(c.parse::<i16>().unwrap());
    }

    return f;
}

fn get_boards(lines: Vec<String>) -> Vec<Vec<Vec<i16>>>
{
    let mut boards: Vec<Vec<Vec<i16>>> = vec![];

    let mut curr_row: Vec<i16>;
    let mut curr_arr: Vec<Vec<i16>> = vec![];

    let mut rows: i8 = 0;

    for (i, line) in lines.into_iter().enumerate()
    {
        if i<2 || line == "" {continue;}

        curr_row = vec![];

        if rows == 5 {
            boards.push(curr_arr);
            curr_arr = vec![];
            rows = 0;
        }

        for n in line.split(" ")
        {
            if n == "" {continue;}
            let n = n.parse::<i16>().unwrap();

            curr_row.push(n);
        }

        curr_arr.push(curr_row);
        rows += 1;
    }

    return boards;
}

fn check_winner(board: Vec<Vec<i16>>, chosen: Vec<i16>) -> bool
{
    for row in &board
    {
        let mut same_row = true;

        for n in row
        {  
            if !chosen.contains(&n) { same_row = false; break; }
        }

        if same_row {return true;}
    }

    for col in 0..board[0].len()
    {
        let mut same_col = true;

        for row in &board
        {
            if !chosen.contains(&row[col]) { same_col = false; break}
        }

        if same_col {return true;}
    }

    return false;
}

fn get_score(board: Vec<Vec<i16>>, chosen: Vec<i16>)
{
    let mut score: i64 = 0;

    for row in &board
    {
        for col in row
        {
            if !chosen.contains(&col) { score += *col as i64; }
        }
    }

    let last = *chosen.last().unwrap();

    println!("Board: {:#?}\nNums: {:#?}\nScore: {}\nLast tried: {}\nResult: {}", board, chosen, score, last, score * last as i64);
}

fn solve(lines: Vec<String>) 
{
    let chosen = get_chosen(lines.clone());
    let mut boards = get_boards(lines.clone());

    let mut curr_chosen: Vec<i16> = vec![];

    for c in &chosen
    {
        curr_chosen.push(*c);

        for board in boards.clone()
        {

            if boards.len() <= 1 && check_winner(board.clone(), curr_chosen.clone()) {
                get_score(board, curr_chosen.clone());
                exit(0);
            }

            if boards.len() > 1 && check_winner(board.clone(), curr_chosen.clone())
            {
                boards.retain(|b| *b != board);
            }
        }
    }
}

fn main() {
    let file: String = "input".to_string();
    let lines: Vec<String> = get_file(file);

    solve(lines);
}
