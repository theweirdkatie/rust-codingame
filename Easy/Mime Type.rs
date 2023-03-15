use std::{io, collections::HashMap};

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
    let n = parse_input!(input_line, i32); // Number of elements which make up the association table.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let q = parse_input!(input_line, i32); // Number Q of file names to be analyzed.
    let mut table = HashMap::<String, String>::new();
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let ext = inputs[0].trim().to_string().to_ascii_lowercase(); // file extension
        let mt = inputs[1].trim().to_string(); // MIME type.
        table.insert(ext, mt);
    }
    for i in 0..q as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let mut fname = input_line.trim_matches('\n').split(".").collect::<Vec<&str>>(); // One file name per line.
        
        if fname.len() > 1 {
            if let Some(ext) = table.get(&fname.pop().unwrap().to_lowercase()) {
                println!("{ext}");
            } else {
                println!("UNKNOWN");
            }
        } else {
            println!("UNKNOWN");
        }
    }
}
