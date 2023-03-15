use std::{io, collections::VecDeque};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut player_1 = parse_cards();
    let mut player_2 = parse_cards();
    let mut pot_1 = VecDeque::new();
    let mut pot_2 = VecDeque::new();
    let mut rounds = 0;

    while player_1.len() > 0 && player_2.len() > 0 {
        // Battle
        let battle_result = player_1[0].0.cmp(&player_2[0].0);

        // Put cards on table
        pot_1.push_back(player_1.pop_front().unwrap());
        pot_2.push_back(player_2.pop_front().unwrap());
        
        match battle_result {
            std::cmp::Ordering::Greater => {
                // player 1 wins
                player_1.append(&mut pot_1);
                player_1.append(&mut pot_2);
                rounds += 1;
            }
            std::cmp::Ordering::Less => {
                // player 2 wins
                player_2.append(&mut pot_1);
                player_2.append(&mut pot_2);
                rounds += 1;
            }
            std::cmp::Ordering::Equal => {
                // Discard for war
                // Check enough cards
                if player_1.len() <3 || player_2.len() < 3 {
                    println!("PAT");
                    return;
                }
                for _ in 0..3 {
                    pot_1.push_back(player_1.pop_front().unwrap());
                    pot_2.push_back(player_2.pop_front().unwrap());
                }
            }
        }
    }
    let winner = if player_1.len() > 0 {1} else {2};
    println!("{winner} {rounds}");
}

fn parse_cards() -> VecDeque<(i32, String)> {
    let mut player = VecDeque::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // the number of cards for player
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        input_line = input_line.trim_matches('\n').to_string();
        let card = input_line.clone().trim_matches(|c| ['D', 'H', 'C', 'S'].contains(&c)).to_string();
        let value = match &card[..] {
            "A" => 14,
            "K" => 13,
            "Q" => 12,
            "J" => 11,
            _ => card.parse().unwrap_or(0),
        };
        player.push_back((value, input_line)); // the n cards of player
    }
    player
}