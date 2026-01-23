use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let raw_input = fs::read_to_string(file).unwrap_or_else(|_| panic!("File {file:?} not found"));
    if !raw_input.is_ascii() {
        panic!("Input must be ASCII only")
    };
    // separate ranges into distinct strings
    let bc_ranges = raw_input.split(",");
    let invalid_id_sum = bc_ranges.fold(0, |acc, el| {
        let (min_str, max_str) = el.split_once("-").expect("Invalid range format");
        let min: u64 = min_str.trim().parse().expect("Invalid number");
        let max: u64 = max_str.trim().parse().expect("Invalid number");
        println!("range: {}-{}", min, max);

        let min_len = min.ilog10() + 1;
        let mut seq_no = if min_len.is_multiple_of(2) {
            let (half1, half2) = min_str.split_at((min_len / 2).try_into().unwrap());
            let half1: u64 = half1.parse().expect("Invalid number");
            let half2: u64 = half2.parse().expect("Invalid number");
            if half1 >= half2 { half1 } else { half1 + 1 }
        } else {
            10_u64.pow(min_len / 2)
        };
        let mut val = seq_no + seq_no * 10_u64.pow(seq_no.ilog10() + 1);
        println!("{}", seq_no);
        let mut sum = acc;
        while val <= max {
            println!("{val}");
            sum += val;
            seq_no += 1;
            val = seq_no + seq_no * 10_u64.pow(seq_no.ilog10() + 1);
        }
        sum
    });
    println!("The sum of all invalid IDs: {invalid_id_sum}")
}
