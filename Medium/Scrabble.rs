use std::{io, collections::HashMap};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    // number of words in dictionary
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    // grab words in dictionary
    let mut words: Vec<String> = vec![];
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let w = input_line.trim_matches('\n').to_string();
        words.push(w);
    }

    // scrabble letters
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let letters = input_line.trim_matches('\n').to_string();

    let mut scrabble_word = String::new();
    let mut scrabble_value: usize = 0;

    for word in words {
        let mut letter_match = letters.clone();
        if word.chars().all(|c| {
            if let Some(idx) = letter_match.find(c) {
                letter_match.remove(idx);
                true
            } else {
                false
            }
        }) {
            let value = find_word_score(&word);
            if value > scrabble_value {
                scrabble_value = value;
                scrabble_word = word.clone();
            }
        }
    }

    println!("{scrabble_word}");
}

fn find_word_score(word: &String) -> usize {
    let mut total = 0;
    for c in word.chars() {
        total += match c {
            'e' | 'a' | 'i' | 'o' | 'n' | 'r' | 't' | 'l' | 's' | 'u' => 1,
            'd' | 'g' => 2,
            'b' | 'c' | 'm' | 'p' => 3,
            'f' | 'h' | 'v' | 'w' | 'y' => 4,
            'k' => 5,
            'j' | 'x' => 8,
            'q' | 'z' => 10,
            _ => 0,
        }
    }
    total
}
