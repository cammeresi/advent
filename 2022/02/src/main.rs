use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

const INPUT: &str = "input.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice {
    Rock = 1,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Loss = 0,
    Tie = 3,
    Win = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ParseChoiceError;

impl FromStr for Choice {
    type Err = ParseChoiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(ParseChoiceError),
        }
    }
}

impl FromStr for Outcome {
    type Err = ParseChoiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Tie),
            "Z" => Ok(Self::Win),
            _ => Err(ParseChoiceError),
        }
    }
}

impl Choice {
    fn battle(&self, x: &Choice) -> Outcome {
        match (self, x) {
            (Choice::Rock, Choice::Rock) => Outcome::Tie,
            (Choice::Paper, Choice::Paper) => Outcome::Tie,
            (Choice::Scissors, Choice::Scissors) => Outcome::Tie,
            (Choice::Rock, Choice::Scissors) => Outcome::Loss,
            (Choice::Scissors, Choice::Paper) => Outcome::Loss,
            (Choice::Paper, Choice::Rock) => Outcome::Loss,
            _ => Outcome::Win,
        }
    }

    fn from_outcome(&self, x: &Outcome) -> Choice {
        match (self, x) {
            (Choice::Rock, Outcome::Tie) => Choice::Rock,
            (Choice::Paper, Outcome::Tie) => Choice::Paper,
            (Choice::Scissors, Outcome::Tie) => Choice::Scissors,
            (Choice::Rock, Outcome::Win) => Choice::Paper,
            (Choice::Paper, Outcome::Win) => Choice::Scissors,
            (Choice::Scissors, Outcome::Win) => Choice::Rock,
            (Choice::Rock, Outcome::Loss) => Choice::Scissors,
            (Choice::Paper, Outcome::Loss) => Choice::Rock,
            (Choice::Scissors, Outcome::Loss) => Choice::Paper,
        }
    }
}

fn read_lines<N>(name: N) -> Vec<String>
where
    N: AsRef<Path> + Display,
{
    let file = match File::open(name) {
        Ok(f) => f,
        Err(e) => panic!("open input error {e}"),
    };
    BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}

fn process(first: bool, lines: Vec<String>) -> u64 {
    let mut total = 0;

    for line in lines {
        let parts = line.split(' ').collect::<Vec<_>>();
        let opponent = parts[0].parse::<Choice>().unwrap();
        let me;

        if first {
            me = parts[1].parse::<Choice>().unwrap();
        } else {
            let outcome = parts[1].parse::<Outcome>().unwrap();
            me = opponent.from_outcome(&outcome);
        }

        total += me as u64 + opponent.battle(&me) as u64;
    }

    total
}

fn process_and_print(first: bool, num: u8, input: &str) {
    let total = process(first, read_lines(input));
    println!("{num}: {total}");
}

fn main () {
    process_and_print(true, 1, INPUT);
    process_and_print(false, 2, INPUT);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "A Y\n\
                           B X\n\
                           C Z\n";

    fn test(first: bool, lines: &str, expect: u64) {
        let lines = lines.lines().map(|x| String::from(x)).collect();
        let total = process(first, lines);
        assert_eq!(total, expect);
    }

    #[test]
    fn example1() {
        test(true, EXAMPLE, 15);
    }

    #[test]
    fn example2() {
        test(false, EXAMPLE, 12);
    }
}
