const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_CRATE_NAME"),
    "_1.txt"
));

fn main() {
    let input: Vec<(&str, i32)> = INPUT
        .split("\n")
        .map(|x| x.split_once(" ").unwrap())
        .map(|(cmd, num)| (cmd, str::parse(num).unwrap()))
        .collect();

    {
        let mut x = 0;
        let mut y = 0;

        for &(cmd, num) in &input {
            match cmd {
                "forward" => x += num,
                "down" => y += num,
                "up" => y -= num,
                _ => unreachable!(),
            }
        }

        let total = x * y;

        dbg!(("part 1", x, y, total));
    }

    {
        let mut x = 0;
        let mut y = 0;
        let mut aim = 0;

        for &(cmd, num) in &input {
            match cmd {
                "forward" => {
                    x += num;
                    y += aim * num;
                },
                "down" => aim += num,
                "up" => aim -= num,
                _ => unreachable!(),
            }
        }

        let total = x * y;

        dbg!(("part 2", x, y, aim, total));
    }
}
