use std::{env, fs};

fn part_a(ranges: Vec<[u64; 2]>, ids: Vec<u64>) -> u32 {
    ids.iter().fold(0, |acc, id| {
        // add one if id in any range
        if ranges
            .iter()
            .any(|range| range[0] <= *id && *id <= range[1])
        {
            acc + 1
        } else {
            acc
        }
    })
}

fn part_b(ranges: Vec<[u64; 2]>) -> u64 {
    // build list of disjoint ranges
    let disjoint_ranges = ranges.iter().fold(vec![], |acc: Vec<[u64; 2]>, el| {
        // could possibly make more efficient by ordering ranges to avoid
        // iterating through all ranges every cycle.
        acc.iter()
            // adjust ranges already in list so they do not overlap with new el -
            // could also adjust el instead.
            // Combining adjacent ranges after an iteration cycle might improve efficiency.
            // Method here always produces to two new ranges per old range but one
            // or both may be empty.
            .flat_map::<[[u64; 2]; 2], _>(|r| {
                [[r[0], r[1].min(el[0] - 1)], [r[0].max(el[1] + 1), r[1]]]
            })
            // remove empty / negative ranges
            .filter(|r| r[0] <= r[1])
            // add new range to list
            .chain([*el])
            .collect()
    });
    disjoint_ranges
        .iter()
        .fold(0, |acc, r| acc + (r[1] + 1) - r[0])
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let raw_input = fs::read_to_string(file).unwrap_or_else(|_| panic!("File {file:?} not found"));
    if !raw_input.is_ascii() {
        panic!("Input must be ASCII only")
    };
    // parse input
    let (raw_ranges, raw_ids) = raw_input.split_once("\n\n").expect("invalid format");
    let ranges: Vec<_> = raw_ranges
        .split("\n")
        .map(|r| {
            r.split_once("-")
                .map(|t| [t.0, t.1])
                .unwrap()
                .map(|v| v.parse::<u64>().expect("unable to convert to int"))
        })
        .collect();
    let ids: Vec<_> = raw_ids
        .split_terminator("\n")
        .map(|v| v.parse::<u64>().expect("unable to convert to int"))
        .collect();

    println!("part a: {}", part_a(ranges.clone(), ids));
    println!("part b: {}", part_b(ranges.clone()))
}
