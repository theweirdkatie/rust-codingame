use std::io;

use itertools::Itertools;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut mines = vec![];
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let w = parse_input!(input_line, usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let h = parse_input!(input_line, usize);
    for i in 0..h {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n');

        // we don't need to store the entire input map, just the locations of the mines
        mines.append(&mut line.char_indices().filter(|(_,c)| *c == 'x').map(|(idx,_)| (i,idx)).collect::<Vec<_>>());
    }

    let mut map = vec![vec![0_usize;w];h];

    for row in 0..h {
        for cell in 0..w {
            // if cell at (row,cell) is NOT a mine, count the number of mines touching
            // Mines touching will always be one unit away
            if !mines.iter().any(|(r,c)| *r==row && *c==cell) {
                map[row][cell] = mines.iter().filter(|(r,c)| {
                    let row_diff = (*r as i32-row as i32).abs();
                    let col_diff = (*c as i32 - cell as i32).abs();
                    row_diff <= 1 && col_diff <= 1
                }).count();
            } else {
                // if the cell contains a mine, make 0
                map[row][cell] = 0;
            }
        }
    }
    
    for row in map {
        // convert all 0's to . and all numbers to string versions
        println!("{}", row.iter().map(|&n| if n==0 {".".to_string()} else {n.to_string()}).collect::<String>());
    }

}
