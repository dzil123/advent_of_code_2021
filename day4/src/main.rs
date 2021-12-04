#![feature(drain_filter)]

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_CRATE_NAME"),
    "_1.txt"
));

const SIZE: usize = 5;
type BoardType<T> = [[T; SIZE]; SIZE];
type BoardRaw = BoardType<i32>;
type BoardBool = BoardType<bool>;

struct Board {
    inner: BoardRaw,
    bool: BoardBool,
}

impl Board {
    fn new(input: &str) -> Self {
        let mut inner: BoardRaw = Default::default();

        for (row_i, row) in input.split("\n").enumerate() {
            for (col_i, item) in row.split_ascii_whitespace().enumerate() {
                inner[row_i][col_i] = item.parse().unwrap();
            }
        }

        Self {
            inner,
            bool: Default::default(),
        }
    }

    fn add_number(&mut self, num: i32) {
        for (row_i, row) in self.inner.iter().enumerate() {
            for (col_i, &item) in row.iter().enumerate() {
                if item == num {
                    self.bool[row_i][col_i] = true;
                }
            }
        }
    }

    fn test_row(&self, row: usize) -> bool {
        for col in 0..SIZE {
            if !self.bool[row][col] {
                return false;
            }
        }

        true
    }

    fn test_col(&self, col: usize) -> bool {
        for row in 0..SIZE {
            if !self.bool[row][col] {
                return false;
            }
        }

        true
    }

    fn test_win(&self) -> bool {
        for i in 0..SIZE {
            if self.test_row(i) || self.test_col(i) {
                return true;
            }
        }

        false
    }

    fn count_unmarked(&self) -> i32 {
        let mut sum = 0;

        for (row_i, row) in self.inner.iter().enumerate() {
            for (col_i, &item) in row.iter().enumerate() {
                if !self.bool[row_i][col_i] {
                    sum += item;
                }
            }
        }

        sum
    }
}

fn main() {
    let mut input = INPUT.split("\n\n");
    let called_numbers: Vec<i32> = input
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut boards: Vec<_> = input.map(Board::new).collect();

    let mut part1 = true;
    let mut last_board = None;

    for &num in &called_numbers {
        for board in boards.drain_filter(|board| {
            board.add_number(num);
            board.test_win()
        }) {
            if part1 {
                println!("part1");
                let unmarked = board.count_unmarked();
                dbg!((num, unmarked));
                dbg!(num * unmarked);
                part1 = false;
            }
            last_board = Some(board)
        }

        if boards.len() == 0 {
            println!("part2");
            let unmarked = last_board.unwrap().count_unmarked();
            dbg!((num, unmarked));
            dbg!(num * unmarked);
            return;
        }
    }
}
