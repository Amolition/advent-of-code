use std::{env, fs};

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
}

fn parse_part_a(raw_input: &str) -> Vec<(Vec<u64>, Op)> {
    let rows: Vec<Vec<_>> = raw_input
        .split_terminator("\n")
        .map(|r| r.split_whitespace().collect())
        .collect();
    let cols: Vec<(Vec<u64>, Op)> = (0..rows[0].len())
        .map(|i| {
            (
                (0..rows.len() - 1)
                    .map(|j| rows[j][i].parse().expect("invalid input format"))
                    .collect(),
                if rows[rows.len() - 1][i] == "+" {
                    Op::Add
                } else {
                    Op::Multiply
                },
            )
        })
        .collect();
    cols
}
fn parse_part_b(raw_input: &str) -> Vec<(Vec<u64>, Op)> {
    let (rows, ops) = raw_input.trim_end().rsplit_once("\n").unwrap();
    let rows: Vec<Vec<_>> = rows.split("\n").map(|r| r.chars().collect()).collect();
    let last_idx = rows
        .iter()
        .fold(0, |v, r| if r.len() > v { r.len() } else { v })
        + 1;
    let ops = ops.chars().enumerate().filter_map(|c| {
        if c.1.is_whitespace() {
            None
        } else {
            let op = match c.1.to_string().as_str() {
                "+" => Op::Add,
                "*" => Op::Multiply,
                _ => panic!("invalid operation"),
            };
            Some((c.0, op))
        }
    });
    let (op_idxs, op_vals): (Vec<_>, Vec<_>) = ops.unzip();
    let ops = op_idxs
        .iter()
        .zip(op_idxs.iter().skip(1).chain([&last_idx]))
        .zip(op_vals);
    let cols: Vec<_> = ops
        .map(|op| {
            let nums: Vec<_> = (*op.0.0..*op.0.1 - 1)
                .map(|i| {
                    let mut num: Vec<char> = vec![];
                    for r in &rows {
                        let i = if i > r.len() - 1 { r.len() - 1 } else { i };
                        num.push(r[i])
                    }
                    num.iter()
                        .collect::<String>()
                        .trim()
                        .parse::<u64>()
                        .unwrap()
                })
                .collect();
            (nums, op.1)
        })
        .collect();
    cols
}

fn solver(cols: Vec<(Vec<u64>, Op)>) -> u64 {
    cols.iter().fold(0, |acc, col| {
        let val = match col.1 {
            Op::Add => col.0.clone().into_iter().reduce(|sum, n| sum + n),
            Op::Multiply => col.0.clone().into_iter().reduce(|prod, n| prod * n),
        }
        .unwrap();
        acc + val
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let raw_input = fs::read_to_string(file).unwrap_or_else(|_| panic!("File {file:?} not found"));
    if !raw_input.is_ascii() {
        panic!("Input must be ASCII only")
    };
    // parse input
    let cols_part_a = parse_part_a(&raw_input);
    let cols_part_b = parse_part_b(&raw_input);

    println!("part a {}", solver(cols_part_a));
    println!("part b {}", solver(cols_part_b));
}
