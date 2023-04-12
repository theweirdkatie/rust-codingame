use std::{io, collections::HashMap};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Defibrillator {
    num: usize,
    name: String,
    address: String,
    phone: String,
    lon: f32,
    lat: f32,
}

impl Defibrillator {
    pub fn new(details: &Vec<&str>) -> Self {
        Self {
            num: details[0].parse::<usize>().expect("Error parsing defib num"),
            name: details[1].to_owned(),
            address: details[2].to_owned(),
            phone: details[3].to_owned(),
            lon: details[4].replace(',', ".").parse::<f32>().expect("Error parsing defib lon"),
            lat: details[5].replace(',', ".").parse::<f32>().expect("Error parsing defib lat"),
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
    let lon = input_line.trim().replace(',', ".").parse::<f32>().expect("Error parsing longitude");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lat = input_line.trim().replace(',', ".").parse::<f32>().expect("Error parsing latitude");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    
    // map to hold defib vals and distance from user
    let mut map: Vec<(Defibrillator,f32)> = vec![];

    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let defib = Defibrillator::new(&input_line.trim_matches('\n').split_terminator(';').collect::<Vec<&str>>());
        let d = find_distance([lon,lat],[defib.lon,defib.lat]);
        map.push((defib, d));
    }

    // find min
    let res = map.iter().min_by(|&a,&b| a.1.partial_cmp(&b.1).unwrap()).unwrap();
    println!("{}", res.0.name);
}

fn find_distance(a: [f32;2], b: [f32;2]) -> f32 {
    // a&b - [lon,lat]
    // x = (b_lon - a_lon) x cos(a_lat+b_lat/2)
    let x = (b[0]-a[0]) * ((a[1]+b[1])/2.0).cos();
    // y = (b_lat - a_lat)
    let y = b[1]-a[1];
    // d = sqrt(x^2 + y^2) x 6371
    (x.powi(2)+y.powi(2)).sqrt() * 6371.0
}