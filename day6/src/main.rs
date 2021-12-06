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

type Num = usize;

const NEW_FISH_DELAY: Num = 8;
const FISH_DELAY: Num = 6;
const DAYS_1: Num = 80;
const DAYS_2: Num = 256;

fn main() {
    let fishes: Vec<Num> = INPUT
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    {
        println!("part1");
        let mut fishes = fishes.clone();
        let mut new_fishes = vec![];

        for _ in 0..DAYS_1 {
            for fish in &mut fishes {
                if *fish == 0 {
                    *fish = FISH_DELAY;
                    new_fishes.push(NEW_FISH_DELAY);
                } else {
                    *fish -= 1;
                }
            }

            fishes.append(&mut new_fishes);
        }

        println!("{}", fishes.len());
    }

    {
        println!("part2");

        let mut fish_ages = VecDeque::from_iter([0; NEW_FISH_DELAY + 2]);
        for fish in fishes {
            fish_ages[fish] += 1;
        }

        for _ in 0..DAYS_2 {
            let new_fish: Num = fish_ages[0];
            fish_ages[NEW_FISH_DELAY + 1] += new_fish;
            fish_ages[FISH_DELAY + 1] += new_fish;

            fish_ages.pop_front();
            fish_ages.push_back(0);
        }

        println!("{}", fish_ages.iter().sum::<usize>());
    }
}
