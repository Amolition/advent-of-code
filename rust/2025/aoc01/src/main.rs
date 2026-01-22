use std::{cmp, env, fs};

/// struct to carry accumulated data through application of rotations
struct CountData {
    pos: u8,
    count_point: u32,
    count_pass: u32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let raw_input = fs::read_to_string(file).unwrap_or_else(|_| panic!("File {file:?} not found"));
    // separate instructions into distinct strings
    let rotations = raw_input.split_whitespace();
    let CountData {
        pos: _,
        count_point,
        count_pass,
    } = rotations.fold(
        // initial data
        CountData {
            pos: 50,
            count_point: 0,
            count_pass: 0,
        },
        |acc, el| {
            // parse instructions into numerical values
            let motion = el
                .replace("L", "-")
                .replace("R", "+")
                .parse::<i16>()
                .unwrap_or_else(|_| panic!("Invalid instruction {el:?}"));
            // find new position after motion
            let new_pos_raw = acc.pos as i16 + motion;
            let new_pos = (new_pos_raw.rem_euclid(100)) as u8;
            // increment count_point if position is zero
            let new_count_point = match new_pos {
                0 => acc.count_point + 1,
                _ => acc.count_point,
            };
            // calculate adjustment to take into account initial position relative to direction of motion
            let adjustment = match acc.pos {
                0 => acc.pos,
                _ => match motion.cmp(&0) {
                    cmp::Ordering::Less => 100 - acc.pos,
                    _ => acc.pos,
                },
            };
            // find number of revolutions including adjustment (i.e. number of times passing zero)
            let pass_count_for_motion = (adjustment as u16 + motion.unsigned_abs()) / 100;
            let new_count_pass = acc.count_pass + pass_count_for_motion as u32;
            CountData {
                pos: new_pos,
                count_point: new_count_point,
                count_pass: new_count_pass,
            }
        },
    );
    println!("zero is pointed to {count_point} times, and passed {count_pass} times")
}
