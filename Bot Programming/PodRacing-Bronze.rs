use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const RADIUS: i32 = 600;

fn main() {
    let mut used_boost = false;
    
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let next_checkpoint_x = parse_input!(inputs[2], i32); // x position of the next check point
        let next_checkpoint_y = parse_input!(inputs[3], i32); // y position of the next check point
        let next_checkpoint_dist = parse_input!(inputs[4], i32); // distance to the next checkpoint
        let next_checkpoint_angle = parse_input!(inputs[5], i32); // angle between your pod orientation and the direction of the next checkpoint
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let opponent_x = parse_input!(inputs[0], i32);
        let opponent_y = parse_input!(inputs[1], i32);
        let mut thrust: usize = 0;

        // checkpoint has a radius of 600 units
        // find the point on the exterior of the radius closest to the pod
        let target = closest_point((&x,&y), (&next_checkpoint_x, &next_checkpoint_y), &RADIUS);
        
        // turn off thrusters if turn angle more than 90 degrees
        if next_checkpoint_angle > 90 || next_checkpoint_angle < -90 {
            thrust = 0;
        // pod can only turn max 18 degrees in a single turn. If the checkpoint is closer than 1000 units
        // with an angle greater than 18, reduce thrust to prevent orbiting
        } else if next_checkpoint_angle >= 18 && next_checkpoint_dist < 1000 {
            thrust = 10;
        } else {
            // adjust thrust based on % grade between 90 and 0
            // never dip below 85 to ensure top speed
            let percent = (15.0 * (next_checkpoint_angle as f32).to_radians().cos()).abs() as usize;
            thrust = 85 + percent;
        }

        // if the checkpoint is dead ahead and the distance is more than 5000 units, use boost
        // prevents using boost on targets that create overshoot
        if next_checkpoint_angle == 0 && next_checkpoint_dist > 5000 && !used_boost {
            println!("{} {} BOOST", target.0, target.1);
            used_boost = true;
        } else {
            println!("{} {} {}", target.0, target.1,thrust);
        }
    }
}

fn closest_point(point: (&i32,&i32), center: (&i32,&i32), r: &i32) -> (i32,i32) {
    let mut c: (f32,f32) = (0.0,0.0);

    // find magnitude of vector v that points from point to center of cirlce
    let v_x = f64::from(point.0 - center.0);
    let v_y = f64::from(point.1 - center.1);
    let mag_v = (v_x.powi(2) + v_y.powi(2)).sqrt();

    // find coordinates of closest point on the circle
    let a_x = f64::from(*center.0) + v_x/mag_v * f64::from(*r/2); 
    let a_y = f64::from(*center.1) + v_y/mag_v * f64::from(*r/2);
    
    (a_x as i32, a_y as i32)

}