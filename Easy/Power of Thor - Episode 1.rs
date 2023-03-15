use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 * ---
 * Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let light_x = parse_input!(inputs[0], i32); // the X position of the light of power
    let light_y = parse_input!(inputs[1], i32); // the Y position of the light of power

    // Thor's initial X and Y positions
    let mut thor = (parse_input!(inputs[2], i32), parse_input!(inputs[3], i32));

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let remaining_turns = parse_input!(input_line, i32); // The remaining amount of turns Thor can move. Do not remove this line.
        let mut direction = String::new();
        
        if thor.1 < light_y {
            direction += "S";
            thor.1 += 1;
        } else if thor.1 > light_y {
            direction += "N";
            thor.1 -= 1;
        }
        if thor.0 < light_x {
            direction += "E";
            thor.0 += 1;
        } else if thor.0 > light_x {
            direction += "W";
            thor.0 -= 1;
        }

        // A single line providing the move to be made: N NE E SE S SW W or NW
        println!("{direction}");
    }
}
