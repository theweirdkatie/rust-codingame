use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let letters = ('A'..='Z').collect::<Vec<char>>();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let l = parse_input!(input_line, usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let h = parse_input!(input_line, usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let t = input_line.trim_matches('\n').to_string().to_ascii_uppercase();
    let mut t_idx = vec![];
    
    for c in t.chars() {
        if let Some(idx) = letters.iter().position(|&x| x==c) {
            t_idx.push(idx);
        } else {
            t_idx.push(letters.len());
        }
    }

    for i in 0..h as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_matches('\n').as_bytes().chunks(l).map(|sl| std::str::from_utf8(sl).unwrap()).collect::<Vec<&str>>();
        let mut res = String::new();
        for i in t_idx.iter() {
            res += row[*i];
        }
        println!("{res}");
    }
}
