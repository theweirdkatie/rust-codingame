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
    let message = input_line.trim_matches('\n').to_string();
    
    // convert message into vector of characters represented by binary numbers
    let char_bytes = message.into_bytes()
                            .iter()
                            .map(|byt| {
                                let mut res = format!("{byt:b}");
                                let diff = 7-res.len();
                                for _ in 0..diff {
                                    res.insert(0, '0');
                                }
                                res
                            })
                            .collect::<Vec<String>>();

    let mut res = String::new();
    let mut prev_char = ' ';
    for c in char_bytes {
        for num in c.chars() {
            if prev_char == num {
                res.push('0');
            } else {
                if res.len() > 0 {
                    res.push(' ')
                }
                prev_char = num;
                if num == '0' { res.push('0'); }
                res.push_str("0 0");
            }
        }
    }
    println!("{res}");
}
