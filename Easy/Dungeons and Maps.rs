use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], usize);
    let h = parse_input!(inputs[1], usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let start_row = parse_input!(inputs[0], usize);
    let start_col = parse_input!(inputs[1], usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut winner = (0, w*h);
    for i in 0..n as usize {
        let mut map = vec![];
        for _ in 0..h as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let map_row = input_line.trim_matches('\n').chars().collect::<Vec<char>>();
            map.push(map_row);
        }
        let (mut row, mut col) = (start_row, start_col);
        let mut steps = 0;
        loop {
            match map[row][col] {
                '^' => if row == 0 { break } else { row -= 1 },     /* move up */
                'v' => if row+1 == h { break } else { row += 1 },   /* move down */
                '<' => if col == 0 { break } else { col -= 1 },     /* move left */
                '>' => if col+1 == w { break } else { col += 1 },   /* move right */
                'T' => {                                            /* treasure */
                    if steps < winner.1 {
                        winner = (i, steps);
                    }
                    break;
                }
                _ => break,                                         /* trap, wall */
            }
            steps += 1;

            // if position = start position, infinite loop
            if row == start_row && col == start_col {
                break;
            }
        }
    }

    if winner.1 == w*h {
        println!("TRAP");
    } else {
        println!("{}", winner.0);
    }
}