use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn get_map() -> HashMap<char, u64> {
    let mut x = 1;
    let mut h = HashMap::new();

    for c in 'a'..='z' {
        h.insert(c, x);
        x += 1;
    }
    for c in 'A'..='Z' {
        h.insert(c, x);
        x += 1;
    }

    h
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

fn process1(lines: &Vec<String>) -> u64 {
    let map = get_map();
    lines
        .iter()
        .map(|x| {
            assert_eq!(x.len() % 2, 0);
            vec![x[0..x.len() / 2].to_string(), x[x.len() / 2..].to_string()]
        })
        .map(|x| {
            x.iter()
                .map(|y| y.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x[0].intersection(&x[1]).collect::<Vec<_>>())
        .flatten()
        .map(|x| map.get(x).unwrap())
        .sum()
}

fn process2(lines: &Vec<String>) -> u64 {
    let map = get_map();
    let mut total = 0;
    let mut bags = lines.iter().map(|x| x.chars().collect::<HashSet<_>>());

    loop {
        let e0 = bags.next();
        let e1 = bags.next();
        let e2 = bags.next();
        let (mut e0, e1, e2) = match (e0, e1, e2) {
            (Some(e0), Some(e1), Some(e2)) => (e0, e1, e2),
            _ => break total,
        };
        e0.retain(|x| e1.contains(x) && e2.contains(x));
        let badge = e0.iter().collect::<Vec<_>>()[0];
        total += map.get(badge).unwrap();
    }
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
    vJrwpWtwJgWrhcsFMMfFFhFp\n\
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
    PmmdzqPrVvPwwTWBwg\n\
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
    ttgJtRGJQctTZtZT\n\
    CrZsJsPPZsGzwwsLwLmpwMDw";

    fn test(t: fn(&Vec<String>) -> u64, expected: u64) {
        let lines = EXAMPLE.lines().map(|x| String::from(x)).collect();
        let actual = t(&lines);
        assert_eq!(actual, expected);
    }

    #[test]
    fn example1() {
        test(process1, 157);
    }

    #[test]
    fn example2() {
        test(process2, 70);
    }
}
