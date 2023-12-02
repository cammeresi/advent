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

fn make_range(x: u64, y: u64) -> HashSet<u64> {
    (x..=y).collect::<HashSet<_>>()
}

fn transform_ranges<T>(r: &Vec<&str>, f: fn(u64, u64) -> T) -> Vec<T> {
    r.iter()
        .map(|x| {
            let x = x.split("-").collect::<Vec<_>>();
            let (x, y) =
                (x[0].parse::<u64>().unwrap(), x[1].parse::<u64>().unwrap());
            f(x, y)
        })
        .collect::<Vec<_>>()
}

fn process1(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|x| x.split(",").collect::<Vec<_>>())
        .map(|x| transform_ranges(&x, make_range))
        .filter(|x| x[0].is_subset(&x[1]) || x[1].is_subset(&x[0]))
        .count()
}

fn overlap(x: (u64, u64), y: (u64, u64)) -> bool {
    x.0 <= y.1 && y.0 <= x.1
}

fn process2(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|x| x.split(",").collect::<Vec<_>>())
        .map(|x| transform_ranges(&x, |y, z| (y, z)))
        .filter(|x| overlap(x[0], x[1]))
        .count()
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
    2-4,6-8\n\
    2-3,4-5\n\
    5-7,7-9\n\
    2-8,3-7\n\
    6-6,4-6\n\
    2-6,4-8";

    fn test(t: fn(&Vec<String>) -> usize, expected: usize) {
        let lines = EXAMPLE.lines().map(|x| String::from(x)).collect();
        let actual = t(&lines);
        assert_eq!(actual, expected);
    }

    #[test]
    fn example1() {
        test(process1, 2);
    }

    #[test]
    fn example2() {
        test(process2, 4);
    }
}
