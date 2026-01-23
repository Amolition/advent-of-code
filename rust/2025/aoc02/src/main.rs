use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let raw_input = fs::read_to_string(file).unwrap_or_else(|_| panic!("File {file:?} not found"));
    if !raw_input.is_ascii() {
        panic!("Input must be ASCII only")
    };
    // separate ranges into distinct strings
    let bc_ranges = raw_input.trim().split(",");
    let invalid_id_sum = bc_ranges.fold(0, |acc, el| {
        let (min_str, max_str) = el.split_once("-").expect("Invalid range format");
        let min: u64 = min_str.parse().expect("Invalid number");
        let max: u64 = max_str.parse().expect("Invalid number");

        let min_len = min.ilog10() + 1;
        let first = if min_len.is_multiple_of(2) {
            let (half1, half2) = min_str.split_at((min_len / 2).try_into().unwrap());
            let half1: u64 = half1.parse().expect("Invalid number");
            let half2: u64 = half2.parse().expect("Invalid number");
            if half1 >= half2 { half1 } else { half1 + 1 }
        } else {
            10_u64.pow(min_len / 2)
        };

        let max_len = max.ilog10() + 1;
        let last = if max_len.is_multiple_of(2) {
            let (half1, half2) = max_str.split_at((max_len / 2).try_into().unwrap());
            let half1: u64 = half1.parse().expect("Invalid number");
            let half2: u64 = half2.parse().expect("Invalid number");
            if half1 <= half2 { half1 } else { half1 - 1 }
        } else {
            10_u64.pow(max_len / 2) - 1
        };

        let mut sum = acc;
        for seq in first..(last + 1) {
            let val = seq + seq * 10_u64.pow(seq.ilog10() + 1);
            sum += val;
        }
        sum
    });
    println!("The sum of all invalid IDs: {invalid_id_sum}")
}
