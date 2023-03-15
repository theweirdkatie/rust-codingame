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
    let w = parse_input!(inputs[0], i32); // width of the building.
    let h = parse_input!(inputs[1], i32); // height of the building.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // maximum number of turns before game over.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let x0 = parse_input!(inputs[0], i32);
    let y0 = parse_input!(inputs[1], i32);
    let mut batman = (x0, y0);
    let mut min = (0,0);
    let mut max = (w-1, h-1);

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let bomb_dir = input_line.trim().to_string(); // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)

        if bomb_dir.contains('R') {
            min.0 = batman.0 + 1;
        } else if bomb_dir.contains('L') {
            max.0 = batman.0 - 1;
        }
        if bomb_dir.contains('D') {
            min.1 = batman.1 + 1;
        } else if bomb_dir.contains('U') {
            max.1 = batman.1 - 1;
        }
        batman = ((min.0 + max.0)/2, (min.1 + max.1)/2);

        // the location of the next window Batman should jump to.
        println!("{} {}", batman.0, batman.1);
    }
}