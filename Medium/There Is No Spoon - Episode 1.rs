use std::io;

use itertools::Itertools;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Don't let the machines win. You are humanity's last hope...
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let width = parse_input!(input_line, i32); // the number of cells on the X axis
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let height = parse_input!(input_line, i32); // the number of cells on the Y axis
    let mut nodes: Vec<(i32, i32)> = vec![];
    for i in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string(); // width characters, each either 0 or .
        for (col, c) in line.char_indices() {
            if c != '.' {
                nodes.push((col as i32,i as i32));
            }
        }
    }

    for node in &nodes {
        let mut right_node = "-1 -1".to_string();
        for i in (node.0+1)..width {
            if nodes.contains(&(i, node.1)) {
                right_node = i.to_string() + " " + &node.1.to_string();
                break;
            }
        }
        let mut down_node = "-1 -1".to_string();
        for i in (node.1+1)..height {
            if nodes.contains(&(node.0, i)) && node.1 < height {
                down_node = node.0.to_string() + " " + &i.to_string();
                break;
            }
        }
        println!("{} {} {right_node} {down_node}", node.0, node.1);
    }

}
