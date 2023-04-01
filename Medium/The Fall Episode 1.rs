use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    Down, Left, Right
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Type0, Type1, Type2, Type3,  Type4,  Type5,  Type6,
    Type7, Type8, Type9, Type10, Type11, Type12, Type13,
}

impl Cell {
    pub fn next(self, dir: Direction) -> Option<Direction> {
        use Direction::*;
        match self {
            Cell::Type0 =>  None,
            Cell::Type1 =>  Some(Down),
            Cell::Type2 =>  match dir {
                                Down => None,
                                d => Some(d),
                            },
            Cell::Type3 =>  if dir==Down { Some(Down) } else { None },
            Cell::Type4 =>  match dir {
                                Down => Some(Left),
                                Left => Some(Down),
                                Right => None,
                            }
            Cell::Type5 =>  match dir {
                                Down => Some(Right),
                                Right => Some(Down),
                                Left => None,
                            },
            Cell::Type6 =>  if dir==Down { None } else { Some(dir) },
            Cell::Type7 =>  if dir!=Right { Some(Down) } else { None },
            Cell::Type8 =>  if dir!=Down { Some(Down) } else { None },
            Cell::Type9 =>  if dir!=Left { Some(Down) } else { None },
            Cell::Type10 => if dir==Down { Some(Left) } else { None },
            Cell::Type11 => if dir==Down { Some(Right) } else { None },
            Cell::Type12 => if dir==Left { Some(Down) } else { None },
            Cell::Type13 => if dir==Right { Some(Down) } else { None },
        }
    }
}

impl From<usize> for Cell {
    fn from(num: usize) -> Self {
        match num {
            1 => Cell::Type1,
            2 => Cell::Type2,
            3 => Cell::Type3,
            4 => Cell::Type4,
            5 => Cell::Type5,
            6 => Cell::Type6,
            7 => Cell::Type7,
            8 => Cell::Type8,
            9 => Cell::Type9,
            10 => Cell::Type10,
            11 => Cell::Type11,
            12 => Cell::Type12,
            13 => Cell::Type13,
            _ => Cell::Type0,
        }
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32); // number of columns.
    let h = parse_input!(inputs[1], i32); // number of rows.
    let mut tunnel = vec![];
    for _ in 0..h as usize {
        let mut row = vec![];
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        for num in input_line.trim_matches('\n').split_ascii_whitespace() { // represents a line in the grid and contains W integers. Each integer represents one room of a given type.
            if let Ok(n) = num.parse::<usize>() {
                row.push(Cell::from(n));
            }
        }
        tunnel.push(row);
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let ex = parse_input!(input_line, i32); // the coordinate along the X axis of the exit (not useful for this first mission, but must be read).

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let xi = parse_input!(inputs[0], usize);
        let yi = parse_input!(inputs[1], usize);
        let pos = match inputs[2].trim() {
            "TOP" => Direction::Down,
            "LEFT" => Direction::Right,
            "RIGHT" => Direction::Left,
            _ => panic!(),
        };

        let next_pos = match tunnel[yi][xi].next(pos) {
            Some(Direction::Down) => (xi, yi+1),
            Some(Direction::Right) => (xi+1, yi),
            Some(Direction::Left) => (xi-1, yi),
            _ => panic!(),
        };

        // One line containing the X Y coordinates of the room in which you believe Indy will be on the next turn.
        println!("{} {}", next_pos.0, next_pos.1);
    }
}
