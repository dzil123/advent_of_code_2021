use bitvec::prelude::*;
use std::{collections::VecDeque, fmt::Debug};

const INPUT_DEMO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_CRATE_NAME"),
    "_demo.txt"
));

const INPUT_CONFIG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_CRATE_NAME"),
    "_config.txt"
));

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_CRATE_NAME"),
    "_1.txt"
));

type Word = BitArr!(for 7, in u8);

fn new_word() -> Word {
    Word::new([(1 << 7) - 1])
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl TryFrom<char> for Segment {
    type Error = char;

    fn try_from(v: char) -> Result<Self, Self::Error> {
        use Segment::*;

        Ok(match v {
            'a' => A,
            'b' => B,
            'c' => C,
            'd' => D,
            'e' => E,
            'f' => F,
            'g' => G,
            _ => return Err(v),
        })
    }
}

impl From<usize> for Segment {
    fn from(s: usize) -> Self {
        use Segment::*;
        match s {
            0 => A,
            1 => B,
            2 => C,
            3 => D,
            4 => E,
            5 => F,
            6 => G,
            _ => unreachable!(),
        }
    }
}

fn to_segments(word: Word) -> Vec<Segment> {
    word.iter_ones().map(Segment::from).collect()
}

#[derive(Debug)]
struct Line {
    words: [Word; 10],
    unknowns: [Word; 4],
}

#[derive(Debug)]
struct Config {
    words: [Word; 10],
    map: [Word; 8],
    rev_map: [i8; 256],
}

// impl Debug for Line {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         fmt.debug_struct("Line")
//             .field("word", &format_args!("{:?}", self.words))
//             // .field("unknowns", &format_args!("{:?}", self.unknowns))
//             .finish()
//     }
// }

fn parse_word(input: &str) -> Word {
    let mut word = Word::zeroed();
    for c in input.chars() {
        let s: Segment = c.try_into().unwrap();
        word.set(s as _, true);
    }
    word
}

macro_rules! parse_words {
    ($n:literal, $input:expr) => {{
        let mut words = [Word::zeroed(); $n];
        _parse_words(&mut $input, &mut words);
        words
    }};
}

fn _parse_words<'a>(input: &mut impl Iterator<Item = &'a str>, words: &mut [Word]) {
    for word in words.iter_mut() {
        *word = parse_word(input.next().unwrap());
    }
}

fn parse_lines(input: &str) -> Vec<Line> {
    let mut input = input.split_ascii_whitespace().peekable();
    let mut lines = vec![];

    loop {
        if input.peek().is_none() {
            break;
        }

        let words = parse_words!(10, input);
        assert_eq!(input.next().unwrap(), "|");
        let unknowns = parse_words!(4, input);
        lines.push(Line { words, unknowns });
    }

    lines
}

fn parse_config(input: &str) -> Config {
    let words = parse_words!(10, input.split_ascii_whitespace());
    let mut map = [Word::zeroed(); 8];

    for word in words {
        map[word.count_ones()] |= word;
    }

    let mut rev_map = [-1; 256];
    for (num, word) in words.iter().enumerate() {
        rev_map[word.as_buffer()[0] as usize] = num as _;
    }

    Config {
        words,
        map,
        rev_map,
    }
}

fn w(x: Word) -> String {
    format!("{:#010b}", x.as_buffer()[0])
}

fn main() {
    let config = parse_config(INPUT_CONFIG);
    // dbg!(config);
    // panic!();

    // let input = INPUT_DEMO;
    let input = INPUT;
    let lines = parse_lines(input);

    let mut x = 0;
    for line in lines {
        for word in line.unknowns {
            if let 2 | 3 | 4 | 7 = word.count_ones() {
                x += 1;
            }
        }
    }
    dbg!(x);

    // for line in &lines {
    //     let mut possibilities = [new_word(); 8]; // map line segment to a list of possible actual segments
    //     for word in line.words {
    //         let len = word.count_ones();
    //         for i in word.iter_ones() {
    //             let before = possibilities[i];
    //             possibilities[i] &= config.map[len];

    //             println!(
    //                 "word: {0:?}\nsegment: {5:?}\npossibilities[{1}] before: {2:?}\nmap[{6}]: {3:?}\npossibilities[{1}] after: {4:?}\n",
    //                 to_segments(word),
    //                 i,
    //                 to_segments(before),
    //                 to_segments(config.map[len]),
    //                 to_segments(possibilities[i]),
    //                 Segment::from(i),
    //                 len
    //             )
    //         }
    //         // let x: Vec<_> = word.iter_ones().collect();
    //         // println!("{:?} {:?}", word, x);
    //     }
    //     dbg!(possibilities);
    //     dbg!(w(possibilities[1]));
    //     // println!("{:?}", possibilities);
    //     panic!()
    // }
}
