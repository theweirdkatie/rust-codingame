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
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let l = parse_input!(inputs[0], i32);
    let h = parse_input!(inputs[1], usize);

    // grab ascii numerals 0-19
    let mut mayan_numerals = vec![String::new();20];
    for i in 0..h {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let top_line = &input_line.trim().chars().collect::<Vec<char>>()[..];
        for (idx, chunk) in top_line.chunks(l as usize).enumerate() {
            if i > 0 {
                mayan_numerals[idx] += "\n";
            }
            mayan_numerals[idx] += &chunk.iter().collect::<String>();
        }
    }

    // grab first number
    let num1 = get_input_number(h);

    // grab second number
    let num2 = get_input_number(h);

    // grab operation
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let operation = input_line.trim();

    // convert number strings to decimal numbers
    let arg1 = convert_mayan_to_decimal(num1, &mayan_numerals);
    let arg2 = convert_mayan_to_decimal(num2, &mayan_numerals);

    // calculate
    let mut res = match operation {
        "+" => arg1 + arg2,
        "-" => arg1.abs_diff(arg2),
        "*" => arg1 * arg2,
        "/" => arg1 / arg2,
        _ => panic!(),
    };

    let mut mayan_res = String::new();
    
    loop {
        mayan_res = mayan_numerals[res % 20].clone() + "\n" + &mayan_res;
        res /= 20;
        if res <= 0 {break;}
    }

    println!("{mayan_res}");
}

fn get_input_number(h: usize) -> Vec<String> {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let len = parse_input!(input_line, usize);
    
    let mut num = vec![String::new()];
    let mut idx: usize = 0;
    for i in 0..len {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        if i > 0 && i % h == 0 {
            num[idx] = num[idx].trim().to_string();
            num.push(String::new());
            idx += 1;
        }
        num[idx] += input_line.trim();
        num[idx] += "\n";
    }
    num
}

fn convert_mayan_to_decimal(num: Vec<String>, mayan_nums: &Vec<String>) -> usize {
    let mut dec = 0;
    let pow = num.len()-1;
    for (i, n) in num.iter().enumerate() {
        if let Some(idx) = mayan_nums.iter().position(|m| n.contains(m)) {
            dec += idx * 20_usize.pow((pow - i) as u32);
        }
    }
    dec
}