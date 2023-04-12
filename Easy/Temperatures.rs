use std::{io, cmp::Ordering};

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
    let n = parse_input!(input_line, i32); // the number of temperatures to analyse
    let mut inputs = String::new();
    let mut min_temp = (100, 100); // min temp, min diff
    io::stdin().read_line(&mut inputs).unwrap();
    let temps = inputs.split_ascii_whitespace().map(|t| parse_input!(t,i32)).collect::<Vec<i32>>();
    let min_temp = temps.into_iter().min_by(|&a,&b| {
        match (0-a).abs().cmp(&(0-b).abs()){
            // if equal, we want the GREATER temp, meaning reverse the returned ordering to MIN
            Ordering::Equal => if a>b {Ordering::Less} else {Ordering::Greater},
            ord => ord,
        }
    }).unwrap_or(0);
    // default to 0 if no min

    println!("{min_temp}");
}
