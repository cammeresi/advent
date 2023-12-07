use std::fs::read_to_string;
use std::iter::zip;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Race {
    time: u64,
    distance: u64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Outcome {
    hold: u64,
    go: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Race {
        Race {
            time: time,
            distance: distance,
        }
    }
}

impl Outcome {
    fn new(hold: u64, go: u64, distance: u64) -> Outcome {
        Outcome {
            hold: hold,
            go: go,
            distance: distance,
        }
    }
}

fn wins<'a>(races: &'a Vec<Race>) -> impl Iterator<Item = usize> + 'a {
    races.iter().map(|r| {
        race(r.time)
            .iter()
            .filter(|o| o.distance > r.distance)
            .count()
    })
}

fn race(time: u64) -> Vec<Outcome> {
    let mut res = Vec::new();
    for hold in 1..time {
        let go = time - hold;
        res.push(Outcome::new(hold, go, hold * go));
    }
    res
}

fn one_line<'a, I>(lines: &mut I) -> Vec<&'a str>
where
    I: Iterator<Item = &'a str>,
{
    let mut l = lines.next().unwrap().split_whitespace();
    l.next();
    l.collect::<Vec<_>>()
}

fn parse(input: &String, join: bool) -> Vec<Race> {
    let mut lines = input.lines();
    let mut races = Vec::new();

    let mut times = one_line(&mut lines);
    let mut dists = one_line(&mut lines);
    let (tstr, dstr);
    if join {
        tstr = times.join("");
        dstr = dists.join("");
        times = vec![&tstr];
        dists = vec![&dstr];
    }
    assert_eq!(times.len(), dists.len());

    let times = times.iter().map(|x| x.parse::<u64>().unwrap());
    let dists = dists.iter().map(|x| x.parse::<u64>().unwrap());

    for (t, d) in zip(times, dists) {
        races.push(Race::new(t, d));
    }
    races
}

fn process1(input: &String) -> usize {
    let races = parse(input, false);
    wins(&races).product()
}

fn process2(input: &String) -> usize {
    let races = parse(input, true);
    let x = wins(&races).next().unwrap();
    x
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

    fn test_example(t: fn(&String) -> usize, expected: usize) {
        let input = super::read_to_string(INPUT).unwrap();
        assert_eq!(t(&input), expected);
    }

    #[test]
    fn example1() {
        test_example(super::process1, 288);
    }

    #[test]
    fn example2() {
        test_example(super::process2, 71503);
    }
}
