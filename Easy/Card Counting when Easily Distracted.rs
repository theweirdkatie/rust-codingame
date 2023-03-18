use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const VALID_ABBREV: [char;13] = ['2','3','4','5','6','7','8','9','T','J','Q','K','A'];
const CARD_VALUES: [usize;13] = [2,3,4,5,6,7,8,9,10,10,10,10,11];

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let stream_of_consciousness = input_line.trim_matches('\n').to_string();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let bust_threshold = parse_input!(input_line, usize);

    // separate stream of consciousness into individual thoughts
    let thoughts = stream_of_consciousness.split_terminator('.').collect::<Vec<_>>();
    let mut card_freq = [0_usize;VALID_ABBREV.len()];

    // examine each thought to see if it is about the cards
    // if it is cards, record the frequency
    for thought in thoughts {
        if thought.chars().all(|c| VALID_ABBREV.contains(&c)) {
            for card in thought.chars() {
                if let Some(idx) = VALID_ABBREV.iter().position(|&c| c==card) {
                    card_freq[idx] += 1;
                }
            }
        }
    }

    // determine number of cards remaining to be dealt, and how many of those won't bust
    let num_cards_remaining = 52 - card_freq.iter().sum::<usize>();
    let num_cards_under_bust: usize = card_freq.iter().enumerate().map(|(idx,&val)|{
        if val < 4 {
            if CARD_VALUES[idx] < bust_threshold || VALID_ABBREV[idx] == 'A' {
                return 4-val;
            }
        }
        return 0;
    }).sum();
    
    // percentage chance = chance of receiving a card that won't bust in remaining cards
    let percentageChance: f32 = (num_cards_under_bust as f32/num_cards_remaining as f32)*100.0;
    println!("{percentageChance:.0}%");
}
