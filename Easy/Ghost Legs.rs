use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut rungs = vec![];
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], usize);
    let h = parse_input!(inputs[1], usize);
    
    // first row is args
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let args = input_line.chars().filter(|&c| !c.is_ascii_whitespace()).collect::<Vec<char>>();

    // map rungs
    for row in 1..h-1 {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();
        rungs.append(&mut line.char_indices().filter(|(_,c)| *c=='-').map(|(col,_)| (row,col)).collect::<Vec<(_,_)>>());
    }

    // last row is labels
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let labels = input_line.chars().filter(|&c| !c.is_ascii_whitespace()).collect::<Vec<char>>();

    // we don't need the map of the actual ladder, just the position of rungs
    // knowing that every turn we go down one row, check to see if a rung exists
    // on that row to the left or right
    // if rung exists, change index
    for arg in args.iter().enumerate() {
        let mut row = 0;
        let mut idx = arg.0*3;
        while row < (h-1) {
            if let Ok(_) = rungs.binary_search(&(row,idx.checked_sub(1).unwrap_or(0))) {
                idx -= 3;
            } else if let Ok(_) = rungs.binary_search(&(row,idx+1)) {
                idx += 3;
            }
            row += 1;
        }
        println!("{}{}", arg.1, labels[idx/3]);
    }
}
