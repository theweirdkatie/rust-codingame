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
    let t = input_line.trim_matches('\n').to_string();
    let mut res = String::new();

    for command in t.split_ascii_whitespace(){
        let mut freq = command.chars().filter(|x| x.is_ascii_digit()).collect::<String>();
        let _char = command.chars().filter(|x| !x.is_ascii_digit()).collect::<String>();
        let mut c = ' ';

        if _char.len() > 0 {
            match &_char[..] {
                "sp" => c = ' ',
                "bS" => c = '\\',
                "sQ" => c = '\'',
                "nl" => c = '\n',
                _ => c = char::from(_char.as_bytes()[0]),
            }
        } else {
            // numeric char
            c = freq.chars().last().unwrap_or_default();
            freq.truncate(freq.len()-1);
        }

        if let Ok(num) = freq.parse::<usize>() {
            for _ in 0..num {
                res.push(c);
            }
        } else {
            res.push(c);
        }
    }

    println!("{res}");
}
