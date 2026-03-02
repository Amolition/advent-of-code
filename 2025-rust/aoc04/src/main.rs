use std::{env, fs};

// Probably would be a lot easier using a library like ndarray
// But tried to process my own 2D array here as nested vec/tuples

fn part_a(grid: Vec<(usize, Vec<(usize, u8)>)>) -> u32 {
    grid.iter().fold(0, |acc1, el1| {
        let sub_rows = grid
            .iter()
            .filter(|y| ((el1.0 as i32 - 1)..(el1.0 as i32 + 2)).contains(&(y.0 as i32)))
            .map(|item| item.clone().1);
        acc1 + el1.1.iter().fold(0, |acc2, el2| {
            if el2.1 == 0 {
                return acc2;
            }
            let sub_grid = sub_rows.clone().map(|y| {
                y.iter()
                    .filter(|x| ((el2.0 as i32 - 1)..(el2.0 as i32 + 2)).contains(&(x.0 as i32)))
                    .map(|item| item.1)
                    .collect::<Vec<_>>()
            });
            let adjacents = sub_grid.flatten();
            let adjacents_sum = adjacents.reduce(|acc3, el3| acc3 + el3).unwrap();
            if adjacents_sum >= 5 { acc2 } else { acc2 + 1 }
        })
    })
}

fn part_b(grid: Vec<(usize, Vec<(usize, u8)>)>) -> u32 {
    #[derive(Clone, Copy)]
    enum CellStates {
        Empty,
        RollTaken,
        RollPresent,
    }
    type Grid = Vec<(usize, Vec<(usize, CellStates)>)>;
    // Convert grid to use CellStates enum
    let grid: Grid = grid
        .iter()
        .map(|i| {
            (
                i.0,
                i.1.iter()
                    .map(|j| {
                        (
                            j.0,
                            match j.1 {
                                1 => CellStates::RollPresent,
                                _ => CellStates::Empty,
                            },
                        )
                    })
                    .collect(),
            )
        })
        .collect();

    fn process_single_cycle(grid: Grid) -> Grid {
        let next_grid: Vec<_> = grid
            .iter()
            .map(|el1| {
                let sub_rows = grid
                    .iter()
                    .filter(|y| ((el1.0 as i32 - 1)..(el1.0 as i32 + 2)).contains(&(y.0 as i32)))
                    .map(|item| item.clone().1);
                (
                    el1.0,
                    el1.1
                        .iter()
                        .map(|el2| {
                            match el2.1 {
                                CellStates::RollPresent => (),
                                _ => return *el2,
                            }
                            let sub_grid = sub_rows.clone().map(|y| {
                                y.iter()
                                    .filter(|x| {
                                        ((el2.0 as i32 - 1)..(el2.0 as i32 + 2))
                                            .contains(&(x.0 as i32))
                                    })
                                    .map(|item| item.1)
                                    .collect::<Vec<_>>()
                            });
                            let adjacents = sub_grid.flatten();
                            let adjacents_sum = adjacents.fold(0, |acc3, el3| match el3 {
                                CellStates::RollPresent => acc3 + 1,
                                _ => acc3,
                            });
                            if adjacents_sum >= 5 {
                                *el2
                            } else {
                                (el2.0, CellStates::RollTaken)
                            }
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        next_grid
    }

    // might be a nicer functional way to do this
    let mut changing = true;
    let mut last_count = 0;
    let mut last_grid = grid;
    while changing {
        last_grid = process_single_cycle(last_grid.clone());
        let new_count = last_grid.iter().fold(0, |acc, el| {
            acc + el.1.iter().fold(0, |acc2, el2| match el2.1 {
                CellStates::RollTaken => acc2 + 1,
                _ => acc2,
            })
        });
        if new_count == last_count {
            changing = false
        }
        last_count = new_count;
    }
    last_count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let raw_input = fs::read_to_string(file).unwrap_or_else(|_| panic!("File {file:?} not found"));
    if !raw_input.is_ascii() {
        panic!("Input must be ASCII only")
    };
    // parse input
    let grid: Vec<(usize, Vec<(usize, u8)>)> = raw_input
        .trim()
        .split("\n")
        .map(|x| {
            x.chars()
                .map(|x| {
                    let at_char = "@".chars().next().unwrap();
                    if x == at_char { 1 } else { 0 }
                })
                .enumerate()
                .collect()
        })
        .enumerate()
        .collect();
    println!("part a: {}", part_a(grid.clone()));
    println!("part b: {}", part_b(grid.clone()));
}
