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
    let r = parse_input!(input_line, usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let l = parse_input!(input_line, usize);

    let mut sequence = vec![r];
    let mut _line = 1;

    while _line < l {
        let mut next_line = vec![];
        let mut seq_iter = sequence.iter();
        let mut prev_num = seq_iter.next().unwrap();
        let mut count = 1;

        while let Some(num) = seq_iter.next() {
            if prev_num == num {
                count += 1;
            } else {
                next_line.push(count);
                next_line.push(*prev_num);
                prev_num = num;
                count = 1;
            }
        }
        next_line.push(count);
        next_line.push(*prev_num);
        sequence = next_line;
        _line += 1;
    }

    println!("{}", sequence.iter().fold(String::new(), |acc,n| acc + &n.to_string() + " ").trim());
}
