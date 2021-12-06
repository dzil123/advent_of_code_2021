use std::mem::swap;

use regex::Regex;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_CRATE_NAME"),
    "_1.txt"
));

#[derive(Debug)]
struct Point {
    x: i16,
    y: i16,
}
#[derive(Debug)]
struct Line(Point, Point);

enum LineType {
    X,
    Y,
    D(Point),
}

impl Line {
    fn new(x0: &str, y0: &str, x1: &str, y1: &str) -> Self {
        let x0 = x0.parse().unwrap();
        let y0 = y0.parse().unwrap();
        let x1 = x1.parse().unwrap();
        let y1 = y1.parse().unwrap();

        Self(Point { x: x0, y: y0 }, Point { x: x1, y: y1 })
    }

    fn is_not_diagonal(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }

    fn points(&self, diagonal: bool) -> impl Iterator<Item = Point> + '_ {
        let (line_type, mut min, mut max) = if self.0.y == self.1.y {
            (LineType::X, self.0.x, self.1.x)
        } else if self.0.x == self.1.x {
            (LineType::Y, self.0.y, self.1.y)
        } else {
            if diagonal {
                (
                    LineType::D(Point {
                        x: (self.1.x - self.0.x).signum(),
                        y: (self.1.y - self.0.y).signum(),
                    }),
                    0,
                    (self.0.x - self.1.x).abs(),
                )
            } else {
                unreachable!()
            }
        };

        if let LineType::D(_) = line_type {
        } else {
            if max < min {
                swap(&mut min, &mut max);
            }
        }

        (min..=max).map(move |v| match &line_type {
            LineType::X => Point { x: v, y: self.0.y },
            LineType::Y => Point { x: self.0.x, y: v },
            LineType::D(dir) => Point {
                x: self.0.x + v * dir.x,
                y: self.0.y + v * dir.y,
            },
        })
    }
}

const SIZE: usize = 1024;
type Board = [[i32; SIZE]; SIZE];

fn run(lines: &[Line], diagonal: bool) {
    let mut board: Board = [[0; SIZE]; SIZE];

    for line in lines
        .iter()
        .filter(|&line| diagonal || line.is_not_diagonal())
    {
        for point in line.points(diagonal) {
            board[point.y as usize][point.x as usize] += 1;
        }
    }

    // for row in &board {
    //     println!("{:?}", row);
    // }

    let mut total = 0;

    for y in 0..SIZE {
        for x in 0..SIZE {
            if board[y][x] >= 2 {
                total += 1;
            }
        }
    }

    println!("{}", total);
}

fn main() {
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();

    let lines: Vec<Line> = INPUT
        .split('\n')
        .filter_map(|line| re.captures(line))
        .map(|cap| Line::new(&cap[1], &cap[2], &cap[3], &cap[4]))
        .collect();

    println!("part1");
    run(&lines, false);

    println!("part2");
    run(&lines, true);
}
