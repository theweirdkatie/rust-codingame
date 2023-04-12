use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

// columns are a-s
// rows are a-y
const COLUMNS: usize = 19;
const ROWS: usize = 25;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

enum Crop {
    Plant,
    Mow,
    PlantMow,
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut field = vec![vec!["{}"; COLUMNS]; ROWS];
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let instructions = input_line.trim_matches('\n').to_string();
    
    for inst in instructions.split_ascii_whitespace() {
        let mut circle = "";
        let mut corn = Crop::Mow;
        if inst.contains("PLANT") {
            if inst.contains("MOW") {
                corn = Crop::PlantMow;
                circle = &inst[8..];
            } else {
                corn = Crop::Plant;
                circle = &inst[5..];
            }
        } else {
            circle = &inst[..];
        }
        // first val is x pos/col index
        let col = ALPHABET.find(&circle[0..1]).unwrap();
        // second val is y pos/row index
        let row = ALPHABET.find(&circle[1..2]).unwrap();
        // last values are the diameter
        let di = circle[2..].parse::<usize>().unwrap();
        for r in 0..field.len() {
            for c in 0..field[0].len() {
                if is_in_circle([r,c], [row,col], di) {
                    let current_crop = field[r][c];
                    match corn {
                        Crop::Plant => field[r][c] = "{}",
                        Crop::Mow => field[r][c] = "  ",
                        Crop::PlantMow => field[r][c] = if current_crop.contains('{') {"  "} else {"{}"},
                    }
                }
            }
        }
    }
    
    for row in field {
        println!("{}", row.join(""));
    }
}

fn is_in_circle(point: [usize;2], center: [usize;2], diameter: usize) -> bool {
    // compute distance between point and the center of the circle
    // d=√((x2 – x1)² + (y2 – y1)²)
    let d = ((point[0]-center[0]).pow(2) as f32 + (point[1]-center[1]).pow(2) as f32).sqrt();
    d < diameter as f32/2.0
}
