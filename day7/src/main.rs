use std::collections::VecDeque;

const INPUT_DEMO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_CRATE_NAME"),
    "_demo.txt"
));

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_CRATE_NAME"),
    "_1.txt"
));

type Num = i32;

fn main() {
    // let crabs: Vec<Num> = INPUT_DEMO
    let crabs: Vec<Num> = INPUT
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let min = crabs.iter().copied().min().unwrap();
    let max = crabs.iter().copied().max().unwrap();

    {
        println!("part1");

        let mut min_fuel = Num::MAX;
        let mut min_pos: i32 = -1;

        for pos in min..=max {
            let mut fuel = 0;
            for &crab in &crabs {
                fuel += (crab - pos).abs();
            }

            if fuel < min_fuel {
                min_fuel = fuel;
                min_pos = pos;
            }
        }

        dbg!((min_fuel, min_pos));
    }

    {
        println!("part2");

        let mut min_fuel = Num::MAX;
        let mut min_pos: i32 = -1;

        for pos in min..=max {
            let mut fuel = 0;
            for &crab in &crabs {
                let dist = (crab - pos).abs();
                fuel += (dist * (dist + 1)) / 2;
            }

            if fuel < min_fuel {
                min_fuel = fuel;
                min_pos = pos;
            }
        }

        dbg!((min_fuel, min_pos));
    }
}
