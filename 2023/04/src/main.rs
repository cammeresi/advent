use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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

fn parse_cards(cards: &Vec<String>) -> Vec<(HashSet<u64>, HashSet<u64>)> {
    let mut out = Vec::new();
    for card in cards {
        let parts = card.split(": ").collect::<Vec<_>>();
        let parts = parts[1]
            .split(" | ")
            .map(|x| {
                x.split_whitespace()
                    .map(|y| y.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        out.push((
            HashSet::from_iter(parts[0].clone()),
            HashSet::from_iter(parts[1].clone()),
        ));
    }
    out
}

fn calc_points(matches: usize) -> u64 {
    if matches == 0 {
        0
    } else {
        2_u64.pow((matches - 1).try_into().unwrap())
    }
}

fn process1(lines: &Vec<String>) -> u64 {
    let cards = parse_cards(&lines);
    let wins = cards
        .iter()
        .map(|x| x.0.intersection(&x.1))
        .collect::<Vec<_>>();
    wins.into_iter().map(|x| calc_points(x.count())).sum()
}

fn process2(lines: &Vec<String>) -> u64 {
    let cards = parse_cards(&lines);
    let wins = cards
        .iter()
        .map(|x| x.0.intersection(&x.1))
        .collect::<Vec<_>>();
    let wins = wins.into_iter().map(|x| x.count()).collect::<Vec<_>>();
    let mut cards = vec![1; wins.len()];
    for i in 0..wins.len() {
        for j in 0..wins[i] {
            cards[i + j + 1] += cards[i];
        }
    }
    cards.iter().sum()
}

fn main() {
    const INPUT: &str = "input.txt";
    let lines = read_lines(INPUT);
    let total = process1(&lines);
    println!("1: {total}");
    let total = process2(&lines);
    println!("2: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    fn const_to_lines(c: &str) -> Vec<String> {
        c.lines().map(|x| String::from(x)).collect()
    }

    fn test_example(t: fn(&Vec<String>) -> u64, expected: u64) {
        let lines = const_to_lines(EXAMPLE);
        assert_eq!(t(&lines), expected);
    }

    #[test]
    fn example1() {
        test_example(process1, 13);
    }

    #[test]
    fn example2() {
        test_example(process2, 30);
    }
}
