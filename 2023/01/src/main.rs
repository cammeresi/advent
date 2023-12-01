use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

const INPUT: &str = "input.txt";

fn read_lines<N>(name: N) -> Lines<BufReader<File>>
where
    N: AsRef<Path> + Display,
{
    let file = match File::open(name) {
        Ok(f) => f,
        Err(e) => panic!("open input error {e}"),
    };
    BufReader::new(file).lines()
}

fn main() {
    let map = vec![
        ("0", 0), ("1", 1), ("2", 2), ("3", 3), ("4", 4),
        ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9),
        ("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4),
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
    ];

    let lines = read_lines(INPUT);
    let mut total = 0;

    for line in lines {
        let mut first = None;
        let mut last = None;
        let line = line.unwrap();

        for i in 0..line.len() {
            for (s, num) in &map {
                if !line[i..].starts_with(s) {
                    continue;
                }
                (first, last) = match (first, last) {
                    (None, _) => (Some(num), Some(num)),
                    (Some(a), _) => (Some(a), Some(num)),
                };
                break;
            }
        }

        total += match (first, last) {
            (Some(a), Some(b)) => 10 * a + b,
            _ => 0,
        };
    }

    println!("{total}");
}
