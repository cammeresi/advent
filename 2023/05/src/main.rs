use itertools::Itertools;
use regex::Regex;
use std::cmp::{min, PartialEq};
use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;
use std::ops::Add;

struct ParseContext {
    seeds: Regex,
    map: Regex,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Diff {
    from: u64,
    to: u64,
    len: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map<'a> {
    dst: &'a str,
    diffs: Vec<Diff>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MapSet<'a> {
    seeds: Vec<u64>,
    maps: HashMap<&'a str, Map<'a>>,
}

struct MultiRangeIterator<T> {
    starts: VecDeque<T>,
    lens: VecDeque<u64>,
    next: Option<T>,
    stop: Option<T>,
    calls: u64,
    total: u64,
}

impl ParseContext {
    fn new() -> ParseContext {
        ParseContext {
            seeds: Regex::new(r"^seeds: (\d+(\s\d+)*)$").unwrap(),
            map: Regex::new(r"^([a-z]+)-to-([a-z]+) map:$").unwrap(),
        }
    }
}

impl<'a> Map<'a> {
    fn new(dst: &'a str) -> Map<'a> {
        Map {
            dst: dst,
            diffs: Vec::new(),
        }
    }

    fn add_range(&mut self, from: u64, to: u64, len: u64) {
        self.diffs.push(Diff {
            from: from,
            to: to,
            len: len,
        });
    }

    fn xlate(&self, num: u64) -> u64 {
        for d in &self.diffs {
            if num >= d.from && num < d.from + d.len {
                return num - d.from + d.to;
            }
        }
        num
    }
}

impl<'a> MapSet<'a> {
    fn new() -> MapSet<'a> {
        MapSet {
            seeds: Vec::new(),
            maps: HashMap::new(),
        }
    }

    fn add(&mut self, src: &'a str, map: Map<'a>) {
        self.maps.insert(src, map);
    }

    fn xlate(&self, src: &'a str, num: u64) -> Option<(&'a str, u64)> {
        match self.maps.get(src) {
            None => None,
            Some(map) => Some((map.dst, map.xlate(num))),
        }
    }
}

impl<T> MultiRangeIterator<T>
where
    T: Copy + PartialEq + Add<u64, Output = T>,
{
    fn new() -> MultiRangeIterator<T> {
        MultiRangeIterator {
            starts: VecDeque::new(),
            lens: VecDeque::new(),
            next: None,
            stop: None,
            calls: 0,
            total: 0,
        }
    }

    fn add(&mut self, start: T, len: u64) {
        assert!(self.next.is_none());
        self.starts.push_back(start);
        self.lens.push_back(len);
        self.total = self.lens.iter().sum();
    }

    fn next_range(&mut self) {
        match self.starts.pop_front() {
            None => self.next = None,
            Some(x) => {
                self.next = Some(x);
                self.stop = Some(x + self.lens.pop_front().unwrap());
            }
        };
    }

    fn progress(&self) {
        if self.calls % 1000000u64 == 0 {
            println!("{} - {}%", self.calls, 100 * self.calls / self.total);
        }
    }
}

impl<T> Iterator for MultiRangeIterator<T>
where
    T: Copy + PartialEq + Add<u64, Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => self.next_range(),
            Some(n) => {
                if n == self.stop.unwrap() {
                    self.next_range();
                }
            }
        };
        match self.next {
            None => None,
            Some(n) => {
                self.calls += 1;
                self.next = Some(n + 1);
                self.progress();
                Some(n)
            }
        }
    }
}

fn parse<'a>(input: &'a String) -> MapSet<'a> {
    let mut input = input.lines();
    let ctx = ParseContext::new();
    let mut set = MapSet::new();

    let seeds = input.next().unwrap();
    let seeds = ctx.seeds.captures(seeds).unwrap().get(1).unwrap().as_str();
    set.seeds = seeds
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(input.next().unwrap(), "");

    while let Some(line) = input.next() {
        let (_, [src, dst]) = ctx.map.captures(line).unwrap().extract();
        let mut map = Map::new(dst);
        while let Some(line) = input.next() {
            if line == "" {
                break;
            }
            let (to, from, len) = line
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();
            map.add_range(from, to, len);
        }
        set.add(src, map);
    }

    set
}

fn run<'a, I>(seeds: I, maps: &'a MapSet) -> u64
where
    I: Iterator<Item = u64>,
{
    let mut best = u64::MAX;

    for s in seeds {
        let mut src = "seed";
        let mut num = s;
        loop {
            match maps.xlate(src, num) {
                None => {
                    best = min(best, num);
                    break;
                }
                Some((dst, n)) => {
                    (src, num) = (dst, n);
                }
            }
        }
    }
    best
}

fn process1(input: &String) -> u64 {
    let maps = parse(input);
    run(maps.seeds.clone().into_iter(), &maps)
}

fn process2(input: &String) -> u64 {
    let maps = parse(input);
    let mut seeds = MultiRangeIterator::new();
    for x in maps.seeds.chunks(2) {
        seeds.add(x[0], x[1]);
    }
    run(seeds, &maps)
}

fn main() {
    const INPUT: &str = "input.txt";
    let input = read_to_string(INPUT).unwrap();
    let total = process1(&input);
    println!("1: {total}");
    let total = process2(&input);
    println!("2: {total}");
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "example.txt";

    fn test_example(t: fn(&String) -> u64, expected: u64) {
        let input = super::read_to_string(INPUT).unwrap();
        assert_eq!(t(&input), expected);
    }

    #[test]
    fn example1() {
        test_example(super::process1, 35);
    }

    #[test]
    fn example2() {
        test_example(super::process2, 46);
    }
}
