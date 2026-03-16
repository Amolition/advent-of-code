use std::{env, fs, iter::Map, str::Chars};

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Beam(u64),
    Splitter,
    Empty,
}

fn process<'a, T: Iterator<Item = Map<Chars<'a>, impl FnMut(char) -> Cell>>>(
    beam_start: Vec<Cell>,
    manifold: T,
) -> Vec<Vec<Cell>> {
    let manifold_width = beam_start.len();
    manifold.fold(vec![beam_start], |acc, r| {
        let last_b = &acc[acc.len() - 1];
        let last_r: Vec<_> = r.collect();
        let next_b = (0..manifold_width)
            .map(|i| {
                let mut count = 0;
                if let Cell::Beam(n) = last_b[i]
                    && last_r[i] != Cell::Splitter
                {
                    count += n
                };
                if i != 0
                    && let Cell::Beam(n) = last_b[i - 1]
                    && last_r[i - 1] == Cell::Splitter
                {
                    count += n;
                }
                if i != manifold_width - 1
                    && let Cell::Beam(n) = last_b[i + 1]
                    && last_r[i + 1] == Cell::Splitter
                {
                    count += n;
                }
                if count > 0 {
                    return Cell::Beam(count);
                }
                Cell::Empty
            })
            .collect();
        [acc, vec![next_b]].concat()
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let raw_input = fs::read_to_string(file).unwrap_or_else(|_| panic!("File {file:?} not found"));
    if !raw_input.is_ascii() {
        panic!("Input must be ASCII only")
    };
    let (beam_start, manifold) = raw_input.split_once("\n").unwrap();
    let beam_start: Vec<_> = beam_start
        .chars()
        .map(|c| match c.to_string().as_str() {
            "." => Cell::Empty,
            "S" => Cell::Beam(1),
            _ => panic!("unexpected char in beam_start"),
        })
        .collect();
    let manifold = manifold.split_terminator("\n").map(|r| {
        r.chars().map(|c| match c.to_string().as_str() {
            "." => Cell::Empty,
            "^" => Cell::Splitter,
            _ => panic!("unexpected char in manifold"),
        })
    });
    let beam = process(beam_start, manifold.clone());
    for b in beam.iter() {
        for c in b {
            match c {
                Cell::Beam(n) => print!("{n}"),
                Cell::Splitter => print!("^"),
                Cell::Empty => print!("."),
            }
        }
        println!()
    }
    let split_count = manifold.skip(1).enumerate().fold(0, |acc, r| {
        r.1.enumerate().fold(acc, |acc, c| {
            if let Cell::Splitter = c.1
                && let Cell::Beam(_) = beam[r.0][c.0]
            {
                return acc + 1;
            }
            acc
        })
    });
    let timeline_count = beam[beam.len() - 1].clone().iter().fold(0, |acc, c| {
        if let Cell::Beam(n) = c {
            return acc + n;
        }
        acc
    });
    println!("part a: {}", split_count);
    println!("part b: {}", timeline_count);
}
