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
    if n > 1 {
        for i in inputs.split_whitespace() {
            let t = parse_input!(i, i32);
            let diff = (t-0).abs();
            match diff.cmp(&min_temp.1) {
                Ordering::Less => min_temp = (t, diff),
                Ordering::Equal => if t >= 0 { min_temp = (t, diff) },
                _ => (),
            }
        }
    } else if n==0 {
        min_temp.0 = 0;
    } else {
        min_temp.0 = parse_input!(inputs, i32);
    }

    println!("{}", min_temp.0);
}
