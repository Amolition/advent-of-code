use std::{env, fs};

// Generate all invalid barcodes in given range by find largest and smallest
fn part_a<'a, T: Iterator<Item = ((&'a str, u64), (&'a str, u64))>>(bc_ranges: T) -> u64 {
    bc_ranges.fold(0, |acc, el| {
        let ((min_str, min), (max_str, max)) = el;
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
    })
}

// checking individual numbers here which is simpler but probably
// less efficient than generating numbers similar to part a solution
fn part_b<'a, T: Iterator<Item = ((&'a str, u64), (&'a str, u64))>>(bc_ranges: T) -> u64 {
    bc_ranges.fold(0, |acc, el| {
        let ((_, min), (_, max)) = el;
        let range_sum = (min..(max + 1)).fold(0, |acc2, el2| {
            let el2_len = el2.ilog10() + 1;
            let bc_invalid = (1..(el2_len / 2 + 1)).any(|digit_count| {
                if !(el2_len.is_multiple_of(digit_count)) {
                    false
                } else {
                    let el2_string = el2.to_string();
                    let el2_chars = el2_string.chars();
                    let el2_chars_offset = el2_string.chars().skip(digit_count.try_into().unwrap());
                    el2_chars.zip(el2_chars_offset).all(|el3| el3.0 == el3.1)
                }
            });
            if bc_invalid { acc2 + el2 } else { acc2 }
        });
        acc + range_sum
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let raw_input = fs::read_to_string(file).unwrap_or_else(|_| panic!("File {file:?} not found"));
    if !raw_input.is_ascii() {
        panic!("Input must be ASCII only")
    };
    // separate ranges into distinct strings
    let bc_ranges = raw_input.trim().split(",");
    let bc_ranges = bc_ranges.map(|el| {
        let (min_str, max_str) = el.split_once("-").expect("Invalid range format");
        let min: u64 = min_str.parse().expect("Invalid number");
        let max: u64 = max_str.parse().expect("Invalid number");
        ((min_str, min), (max_str, max))
    });
    let invalid_id_sum_part_a = part_a(bc_ranges.clone());
    println!("The sum of all invalid IDs (part a): {invalid_id_sum_part_a}");
    let invalid_id_sum_part_b = part_b(bc_ranges.clone());
    println!("The sum of all invalid IDs (part b): {invalid_id_sum_part_b}");
}
