use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut horses = vec![];
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        horses.push(parse_input!(input_line, i32));
    }

    horses.sort();
    let mut diff = *horses.last().unwrap();

    for chunk in horses.windows(2) {
        if (chunk[0] - chunk[1]).abs() < diff {
            diff = (chunk[0] - chunk[1]).abs();
        }
    }

    println!("{diff}");
}
