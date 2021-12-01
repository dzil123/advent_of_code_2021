const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_CRATE_NAME"),
    "_1.txt"
));

fn part1(iter: impl Iterator<Item = i32>) -> i32 {
    let mut count = 0;
    iter.reduce(|a: i32, b| {
        if b > a {
            count += 1;
        }
        b
    })
    .unwrap();
    count
}

fn main() {
    let input: Vec<i32> = INPUT
        .split_whitespace()
        .map(|x| str::parse(x).unwrap())
        .collect();

    {
        let count = part1(input.iter().copied());
        println!("part 1: {}", count);
    }

    {
        let max_window_size = 3;

        let iter = input
            .windows(max_window_size)
            .map(|x| x.iter().copied().sum());

        let count = part1(iter);
        println!("part 2: {}", count);
    }
}
