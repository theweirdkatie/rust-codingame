use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const TARGET_VELOCITY: i32 = -40;
const GRAVITY: i32 = -4;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let surface_n = parse_input!(input_line, i32); // the number of points used to draw the surface of Mars.
    for _ in 0..surface_n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let land_x = parse_input!(inputs[0], i32); // X coordinate of a surface point. (0 to 6999)
        let land_y = parse_input!(inputs[1], i32); // Y coordinate of a surface point. By linking all the points together in a sequential fashion, you form the surface of Mars.
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let h_speed = parse_input!(inputs[2], i32); // the horizontal speed (in m/s), can be negative.
        let v_speed = parse_input!(inputs[3], i32); // the vertical speed (in m/s), can be negative.
        let fuel = parse_input!(inputs[4], i32); // the quantity of remaining fuel in liters.
        let rotate = parse_input!(inputs[5], i32); // the rotation angle in degrees (-90 to 90).
        let power = parse_input!(inputs[6], i32); // the thrust power (0 to 4).

        let thrust = (TARGET_VELOCITY - GRAVITY - v_speed).min(4).max(0);
        eprintln!("{TARGET_VELOCITY} - {GRAVITY} - {v_speed}");
        // 2 integers: rotate power. rotate is the desired rotation angle (should be 0 for level 1), power is the desired thrust power (0 to 4).
        println!("0 {thrust}");
    }
}
