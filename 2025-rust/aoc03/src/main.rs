use std::{env, fs};

fn part_a<T: Iterator<Item: Iterator<Item = u32>>>(banks: T) -> u32 {
    banks.fold(0, |acc1, el1| {
        let el1: Vec<_> = el1.collect();
        let (first_idx, first) = el1[..el1.len() - 1]
            .iter()
            .enumerate()
            .reduce(|curr, next| if curr.1 >= next.1 { curr } else { next })
            .expect("first_iter was empty");
        let second = el1[(first_idx + 1)..].iter().max().unwrap();
        println!("{}", first * 10 + second);
        acc1 + first * 10 + second
    })
}

fn part_b<T: Iterator<Item: Iterator<Item = u32>>>(banks: T) -> u64 {
    banks.fold(0, |acc1, el1| {
        let el1: Vec<_> = el1.collect();
        let digits: u64 = (0..12)
            .rev()
            .fold((el1, vec![0]), |acc2, el2| {
                let (digit_idx, digit) = acc2.0[..(acc2.0.len() - el2)]
                    .iter()
                    .enumerate()
                    .reduce(|curr, next| if curr.1 >= next.1 { curr } else { next })
                    .expect("iter was empty");
                (
                    // remaining bank after chosen digit to search for next digit
                    acc2.0[digit_idx + 1..].to_vec(),
                    [acc2.1, vec![*digit]].concat(),
                )
            })
            .1
            // convert vec of digits to number
            .iter()
            .map(|x| x.to_string())
            .collect::<String>()
            .parse()
            .expect("unable to parse string to number");
        acc1 + digits
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
    let banks = raw_input.trim().split("\n");
    let banks = banks.map(|el| {
        el.chars()
            .map(|el| el.to_digit(10).expect("invalid char for digit"))
    });
    println!("part a: {}", part_a(banks.clone()));
    println!("part b: {}", part_b(banks.clone()));
}
