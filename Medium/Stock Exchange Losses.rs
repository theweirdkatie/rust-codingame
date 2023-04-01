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
    let n = parse_input!(input_line, usize);
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();
    let mut stocks = vec![];
    for i in inputs.split_whitespace() {
        let v = parse_input!(i, i32);
        stocks.push(v);
    }

    let price_diffs = stocks.windows(2).map(|x| x[1]-x[0]).collect::<Vec<i32>>();
    let mut max_losses = vec![price_diffs[0]];

    for i in 1..n-1 {
        max_losses.push(price_diffs[i].min(max_losses[i-1] + price_diffs[i]));
    }

    let min = max_losses.iter().min().unwrap_or(&0).min(&0);

    println!("{min}");
}
