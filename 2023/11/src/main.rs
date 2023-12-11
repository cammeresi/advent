use std::cmp::{max, min};
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Space {
    Empty,
    Galaxy,
}

fn parse_space(c: char) -> Space {
    match c {
        '.' => Space::Empty,
        '#' => Space::Galaxy,
        _ => panic!("unknown space"),
    }
}

fn parse(input: &String) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|x| x.chars().map(parse_space).collect())
        .collect()
}

fn find_empty_rows(map: &Vec<Vec<Space>>) -> Vec<usize> {
    let mut rows = Vec::new();
    for y in 0..map.len() {
        let mut total = 0;
        for x in 0..map[0].len() {
            if map[y][x] == Space::Galaxy {
                total += 1;
            }
        }
        if total == 0 {
            rows.push(y);
        }
    }
    rows
}

fn find_empty_cols(map: &Vec<Vec<Space>>) -> Vec<usize> {
    let mut cols = Vec::new();
    for x in 0..map[0].len() {
        let mut total = 0;
        for y in 0..map.len() {
            if map[y][x] == Space::Galaxy {
                total += 1;
            }
        }
        if total == 0 {
            cols.push(x);
        }
    }
    cols
}

fn path(
    a: (usize, usize), b: (usize, usize), expand: usize, rows: &Vec<usize>,
    cols: &Vec<usize>,
) -> usize {
    let (x0, x1) = (min(a.0, b.0), max(a.0, b.0));
    let (y0, y1) = (min(a.1, b.1), max(a.1, b.1));
    let mut dist = x1 - x0 + y1 - y0;
    for r in rows {
        if y0 < *r && *r < y1 {
            dist += expand - 1;
        }
    }
    for c in cols {
        if x0 < *c && *c < x1 {
            dist += expand - 1;
        }
    }
    dist
}

fn find_gals(map: &Vec<Vec<Space>>) -> Vec<(usize, usize)> {
    let mut gals = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == Space::Galaxy {
                gals.push((x, y));
            }
        }
    }
    gals
}

fn run(input: &String, expand: usize) -> usize {
    let map = parse(input);
    let rows = find_empty_rows(&map);
    let cols = find_empty_cols(&map);
    let gals = find_gals(&map);

    let mut total = 0;
    for a in 0..gals.len() {
        for b in a + 1..gals.len() {
            total += path(gals[a], gals[b], expand, &rows, &cols);
        }
    }

    total
}

fn process1(input: &String) -> usize {
    run(input, 2)
}

fn process2(input: &String) -> usize {
    run(input, 1000000)
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
    use super::*;

    const EXAMPLE: &str = "example.txt";

    #[test]
    fn example1() {
        let input = read_to_string(EXAMPLE).unwrap();
        assert_eq!(process1(&input), 374);
    }

    #[test]
    fn example2() {
        let input = read_to_string(EXAMPLE).unwrap();
        assert_eq!(run(&input, 10), 1030);
    }

    #[test]
    fn example3() {
        let input = read_to_string(EXAMPLE).unwrap();
        assert_eq!(run(&input, 100), 8410);
    }
}
