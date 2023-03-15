use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
 #[derive(Debug)]
pub struct Cell {
    operation: String,
    arg1: String,
    arg2: String,
    value: Option<i32>,
}

fn main() {
    let mut sheet: Vec<Cell> = vec![];
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        sheet.push(Cell::new(inputs[0], inputs[1], inputs[2]));   
    }
    
    for i in 0..sheet.len() {
        println!("{}", calculate(&mut sheet, i));
    }
}
fn calculate(sheet: &mut Vec<Cell>, i: usize) -> i32 {
    let arg1 = sheet[i].arg1.clone();
    let arg2 = sheet[i].arg2.clone();
    if sheet[i].value.is_none() {
        sheet[i].value = Some(match sheet[i].operation.as_str() {
            "VALUE" => get_arg(sheet, &arg1),
            "ADD" => get_arg(sheet, &arg1) + get_arg(sheet, &arg2),
            "SUB" => get_arg(sheet, &arg1) - get_arg(sheet, &arg2),
            "MULT" => get_arg(sheet, &arg1) * get_arg(sheet, &arg2),
            _ => todo!(),
        })
    }
    sheet[i].value.unwrap()
}

fn get_arg(sheet: &mut Vec<Cell>, arg: &str) -> i32 {
    if arg.contains('$') {
        let idx = arg[1..].parse::<usize>().unwrap_or(0);
        calculate(sheet, idx)
    } else {
        arg.parse::<i32>().unwrap_or(0)
    }
}

impl Cell {
    pub fn new(ops: &str, a1: &str, a2: &str) -> Self {
        Self{
            operation: ops.trim().to_string(),
            arg1: a1.trim().to_string(),
            arg2: a2.trim().to_string(),
            value: None,
        }
    }
}