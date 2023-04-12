use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const ALPHABET: [char;26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let operation = input_line.trim_matches('\n');
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let pseudo_random_number = parse_input!(input_line, usize);
    let mut rotors = vec![];
    for _ in 0..3 as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let rotor = input_line.trim_matches('\n').chars().collect::<Vec<char>>();
        rotors.push(rotor);
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let message = input_line.trim_matches('\n').to_string();

    match operation {
        "ENCODE" => encode_message(&message, &pseudo_random_number, &rotors),
        "DECODE" => decode_message(&message, &pseudo_random_number, &rotors),
        _ => panic!(),
    }
}

fn encode_message(message: &String, shift: &usize, rotors: &Vec<Vec<char>>) {
    // first, shift each letter in message by n + i, making sure to roll over at 26
    let mut encoded_message = message.chars().enumerate().map(|(i,c)|{
        let mut idx = ALPHABET.iter().position(|&a| a==c).unwrap();
        idx = (idx + shift + i) % 26;
        ALPHABET[idx]
    }).collect::<String>();

    // apply each rotor
    for rotor in rotors {
        encoded_message = encoded_message.chars().map(|c|{
            let idx = ALPHABET.iter().position(|&a| a==c).unwrap();
            rotor[idx]
        }).collect::<String>();
    }

    // print result
    println!("{encoded_message}");
}

fn decode_message(message: &String, shift: &usize, rotors: &Vec<Vec<char>>) {
    // go backwards from encode
    let mut decoded_message = message.clone();

    // apply rotors backwards
    for rotor in rotors.iter().rev() {
        decoded_message = decoded_message.chars().map(|c|{
            let idx = rotor.iter().position(|&a| a==c).unwrap();
            ALPHABET[idx]
        }).collect::<String>();
    }

    // then, shift message backwards by n - i, making sure to roll over negatives
    decoded_message = decoded_message.chars().enumerate().map(|(i,c)|{
        let mut idx = ALPHABET.iter().position(|&a| a==c).unwrap();
        idx = ((idx as i32 - *shift as i32 - i as i32).rem_euclid(26)) as usize;
        ALPHABET[idx]
    }).collect::<String>();

    println!("{decoded_message}");
}
