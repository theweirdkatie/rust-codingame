use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Signal{
    name: String,
    bits: Vec<bool>,
}

struct Commands {
    name: String,
    operation: Operations,
    arg1: String,
    arg2: String,
}

enum Operations {
    AND,
    OR,
    XOR,
    NAND,
    NOR,
    NXOR,
}
/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let m = parse_input!(input_line, i32);

    let mut signals: Vec<Signal> = vec![];
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        signals.push(Signal::new(inputs[0].trim(), inputs[1].trim()));
    }

    let mut commands: Vec<Commands> = vec![];
    for _ in 0..m as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        commands.push(Commands::new(inputs[0].trim(), inputs[1].trim(), inputs[2].trim(), inputs[3].trim()));
    }
    
    for i in 0..m as usize {
        let arg1 = signals.iter().find(|s| s.name == commands[i].arg1).unwrap();
        let arg2 = signals.iter().find(|s| s.name == commands[i].arg2).unwrap();
        let mut out = String::new();
        for bits in arg1.bits.iter().zip(arg2.bits.iter()) {
            let b = match commands[i].operation {
                Operations::AND => *bits.0 && *bits.1,
                Operations::OR => *bits.0 || *bits.1,
                Operations::XOR => *bits.0 ^ *bits.1,
                Operations::NAND => !(*bits.0 && *bits.1),
                Operations::NOR => !(*bits.0 || *bits.1),
                Operations::NXOR => !(*bits.0 ^ *bits.1),
            };
            out.push(if b {'-'} else {'_'});
        }
        println!("{} {}", commands[i].name, out);
    }
}

impl Signal {
    pub fn new(n: &str, bits: &str) -> Self {
        Self{ 
            name: n.to_owned(), 
            bits: bits.chars().map(|c| if c=='-' {true} else {false}).collect::<Vec<bool>>(), 
        }
    }
}

impl Commands {
    pub fn new(nm: &str, op: &str, arg_1: &str, arg_2: &str) -> Self {
        Self{ 
            name: nm.to_owned(), 
            arg1: arg_1.to_owned(), 
            arg2: arg_2.to_owned(), 
            operation: match op{
                "AND" => Operations::AND,
                "OR" => Operations::OR,
                "XOR" => Operations::XOR,
                "NAND" => Operations::NAND,
                "NOR" => Operations::NOR,
                "NXOR" => Operations::NXOR,
                _ => panic!(),
            },
        }
    }
}