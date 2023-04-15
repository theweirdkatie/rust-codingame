use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut enemies: Vec<(usize, usize)> = vec![];
    let mut cannon_idx = 0;
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        for (idx, c) in input_line.trim().char_indices() {
            match c {
                '>' | '<' => enemies.push((idx,i)),
                '^' => cannon_idx = idx,
                _ => (),
            }
        }
    }

    let mut turns = vec![];
    for enemy in enemies {
        let delta_x = enemy.0.abs_diff(cannon_idx);
        let delta_y = n as usize-1-enemy.1;
        turns.push(delta_x.abs_diff(delta_y));
    }

    turns.sort_unstable();
    let _max = *turns.last().unwrap();
    for i in 1..=_max {
        if turns.contains(&i) {
            println!("SHOOT");
        } else {
            println!("WAIT");
        }
    }
}
