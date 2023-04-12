use std::{io, convert::TryInto};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Debug, PartialEq)]
enum Direction {
    LEFT,
    RIGHT,
    NONE,
}
/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let nb_floors = parse_input!(inputs[0], i32); // number of floors
    let width = parse_input!(inputs[1], i32); // width of the area
    let nb_rounds = parse_input!(inputs[2], i32); // maximum number of rounds
    let exit_floor = parse_input!(inputs[3], usize); // floor on which the exit is found
    let exit_pos = parse_input!(inputs[4], usize); // position of the exit on its floor
    let nb_total_clones = parse_input!(inputs[5], i32); // number of generated clones
    let nb_additional_elevators = parse_input!(inputs[6], i32); // ignore (always zero)
    let nb_elevators = parse_input!(inputs[7], i32); // number of elevators

    let mut map = vec![None;nb_floors as usize];
    map[exit_floor] = Some(exit_pos);
    for _ in 0..nb_elevators as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let elevator_floor = parse_input!(inputs[0], usize); // floor on which this elevator is found
        let elevator_pos = parse_input!(inputs[1], usize); // position of the elevator on its floor

        // level 1 only cares about elevators and exits
        // don't need to know about previous state
        map[elevator_floor] = Some(elevator_pos);
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let clone_floor = parse_input!(inputs[0], i32); // floor of the leading clone
        let clone_pos = parse_input!(inputs[1], i32); // position of the leading clone on its floor
        let direction = match inputs[2].trim() {
            "LEFT" => Direction::LEFT,
            "RIGHT" => Direction::RIGHT,
            _ => Direction::NONE,
        }; // direction of the leading clone: LEFT or RIGHT

        let mut command = "WAIT";
        if clone_floor >= 0 {
            // check directions are equal
            let current_exit = map[clone_floor as usize].unwrap_or(clone_pos as usize);
            if direction == Direction::LEFT && current_exit > clone_pos as usize ||
            direction == Direction::RIGHT && current_exit < clone_pos as usize {
                // going wrong direction
                // block
                command = "BLOCK";
                // if came from elevator, remove
                if clone_floor != 0 {
                    let prev_floor = (clone_floor-1) as usize;
                    if let Some(elevator) = map[prev_floor] {
                        if elevator == clone_pos as usize {
                            map[prev_floor] = None;
                        }
                    }
                }
            }
        }

        println!("{command}"); // action: WAIT or BLOCK
    }
}
