const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_CRATE_NAME"),
    "_1.txt"
));

fn part2(candidates: &[&str], common: bool) -> String {
    let mut candidates = Vec::from(candidates);

    for i in 0..candidates[0].len() {
        let total: usize = candidates
            .iter()
            .map(|&s| s.as_bytes()[i] as usize - '0' as usize)
            .sum();

        let reference = if common {
            if total >= (candidates.len() - total) {
                '1'
            } else {
                '0'
            }
        } else {
            if total < (candidates.len() - total) {
                '1'
            } else {
                '0'
            }
        };

        candidates.retain(|&s| s.as_bytes()[i] == reference as _);

        if candidates.len() <= 1 {
            return candidates[0].into();
        }
    }

    unreachable!()
}

fn show(x: &str, y: &str) {
    let x = i32::from_str_radix(&x, 2).unwrap();
    let y = i32::from_str_radix(&y, 2).unwrap();

    println!("{}", x * y);
}

fn main() {
    let input: Vec<&str> = INPUT.split("\n").collect();
    let len = input.len();
    let width = input[0].len();

    let (gamma, epsilon) = {
        let mut totals = vec![0; width];

        for &s in &input {
            for (i, c) in s.char_indices() {
                match c {
                    '0' => {}
                    '1' => totals[i] += 1,
                    _ => unreachable!(),
                }
            }
        }

        let gamma: String = totals
            .iter()
            .map(|&i| if i > len / 2 { '1' } else { '0' })
            .collect();

        let epsilon: String = totals
            .iter()
            .map(|&i| if i <= len / 2 { '1' } else { '0' })
            .collect();

        (gamma, epsilon)
    };

    {
        println!("part1");
        show(&gamma, &epsilon);
    }

    {
        println!("part2");

        let o2 = part2(&input, true);
        let co2 = part2(&input, false);

        show(&o2, &co2);
    }
}
