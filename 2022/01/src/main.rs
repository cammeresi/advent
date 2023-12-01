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
    let lines = read_lines(INPUT);
    let mut totals = Vec::new();
    let mut total = 0;

    for line in lines {
        let line = line.unwrap();
        if line != "" {
            total += line.parse::<u64>().unwrap();
        } else {
            totals.push(total);
            total = 0;
        }
    }

    totals.sort_by(|a, b| b.cmp(a));
    let mut top3 = 0;
    for i in 0..3 {
        println!("{}", totals[i]);
        top3 += totals[i];
    }
    println!("top 3 total: {top3}");
}
