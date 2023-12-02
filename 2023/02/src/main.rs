use std::cmp::max;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pick {
    red: u64,
    green: u64,
    blue: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u64,
    picks: Vec<Pick>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ParsePickError;

impl FromStr for Pick {
    type Err = ParsePickError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colors = s
            .split(", ")
            .map(|c| c.split(" ").collect::<Vec<_>>())
            .map(|c| (c[1], c[0].parse().unwrap()))
            .collect::<HashMap<_, _>>();
        Ok(Pick {
            red: *colors.get("red").unwrap_or(&0),
            green: *colors.get("green").unwrap_or(&0),
            blue: *colors.get("blue").unwrap_or(&0),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, picks) = s
            .strip_prefix("Game ")
            .and_then(|s| s.split_once(": "))
            .ok_or(ParseGameError)?;
        let picks = picks.split("; ").map(|p| p.parse().unwrap()).collect();
        Ok(Game {
            id: id.parse().unwrap(),
            picks: picks,
        })
    }
}

impl Game {
    fn compat(&self, have: &Pick) -> bool {
        for p in &self.picks {
            if p.red > have.red || p.green > have.green || p.blue > have.blue {
                return false;
            }
        }
        true
    }

    fn power(&self) -> u64 {
        let r = self.picks.iter().fold(0, |r, p| max(r, p.red));
        let g = self.picks.iter().fold(0, |g, p| max(g, p.green));
        let b = self.picks.iter().fold(0, |b, p| max(b, p.blue));
        r * g * b
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

fn process1(lines: &Vec<String>) -> u64 {
    let have = Pick {
        red: 12,
        green: 13,
        blue: 14,
    };
    lines
        .iter()
        .map(|x| x.parse::<Game>().unwrap())
        .filter_map(|x| if x.compat(&have) { Some(x.id) } else { None })
        .sum()
}

fn process2(lines: &Vec<String>) -> u64 {
    lines
        .iter()
        .fold(0, |t, x| t + x.parse::<Game>().unwrap().power())
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
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    fn test(t: fn(&Vec<String>) -> u64, expected: u64) {
        let lines = EXAMPLE.lines().map(|x| String::from(x)).collect();
        let actual = t(&lines);
        assert_eq!(actual, expected);
    }

    #[test]
    fn example1() {
        test(process1, 8);
    }

    #[test]
    fn example2() {
        test(process2, 2286);
    }
}
