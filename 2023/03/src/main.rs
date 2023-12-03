use std::cmp::min;
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

fn split_grid(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn find_symbols(
    grid: &Vec<Vec<char>>, only: Option<char>,
) -> HashSet<(usize, usize)> {
    let (len_x, len_y) = (grid[0].len(), grid.len());
    let mut syms = HashSet::new();
    for y in 0..len_y {
        for x in 0..len_x {
            if !grid[y][x].is_ascii_digit()
                && grid[y][x] != '.'
                && (only == None || only.unwrap() == grid[y][x])
            {
                syms.insert((x, y));
            }
        }
    }
    syms
}

fn find_coords(
    grid: &Vec<Vec<char>>, syms: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let (len_x, len_y) = (grid[0].len(), grid.len());
    let mut check = HashSet::new();
    for (x, y) in syms {
        for i in (x.saturating_sub(1))..=min(x + 1, len_x - 1) {
            for j in (y.saturating_sub(1))..=min(y + 1, len_y - 1) {
                check.insert((i, j));
            }
        }
        check.remove(&(*x, *y));
    }
    check
}

fn get_part(
    grid: &Vec<Vec<char>>, check: &mut HashSet<(usize, usize)>, x: usize,
    y: usize,
) -> Option<u64> {
    if !check.contains(&(x, y)) || !grid[y][x].is_ascii_digit() {
        None
    } else {
        let (mut left, mut right) = (x, x);
        for x in (0..=left).rev() {
            if !grid[y][x].is_ascii_digit() {
                break;
            }
            left = x;
        }
        for x in right..grid[0].len() {
            if !grid[y][x].is_ascii_digit() {
                break;
            }
            right = x;
        }
        let mut part = 0;
        for x in left..=right {
            part = 10 * part + grid[y][x].to_digit(10).unwrap();
            check.remove(&(x, y));
        }
        Some(part.into())
    }
}

fn find_parts(
    grid: &Vec<Vec<char>>, check: &mut HashSet<(usize, usize)>,
) -> Vec<u64> {
    let mut parts = Vec::new();
    for (x, y) in check.clone() {
        if let Some(p) = get_part(grid, check, x, y) {
            parts.push(p);
        }
    }
    parts
}

fn process1(lines: &Vec<String>) -> u64 {
    let grid = split_grid(lines);
    let syms = find_symbols(&grid, None);
    let mut check = find_coords(&grid, &syms);
    let parts = find_parts(&grid, &mut check);
    parts.iter().sum()
}

fn process2(lines: &Vec<String>) -> u64 {
    let grid = split_grid(lines);
    let syms = find_symbols(&grid, Some('*'));
    let mut gears = Vec::new();
    for s in syms {
        let mut check = find_coords(&grid, &HashSet::from([s]));
        let parts = find_parts(&grid, &mut check);
        if parts.len() == 2 {
            gears.push(parts[0] * parts[1]);
        }
    }
    gears.iter().sum()
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

    const SIMPLE: &str = "*2.\n\
                          100";

    const EXAMPLE: &str = "\
    467..114..\n\
    ...*......\n\
    ..35..633.\n\
    ......#...\n\
    617*......\n\
    .....+.58.\n\
    ..592.....\n\
    ......755.\n\
    ...$.*....\n\
    .664.598..";

    fn const_to_lines(c: &str) -> Vec<String> {
        c.lines().map(|x| String::from(x)).collect()
    }

    #[test]
    fn test_simple() {
        let lines = const_to_lines(SIMPLE);
        let grid = split_grid(&lines);
        let syms = find_symbols(&grid, None);
        assert_eq!(syms, HashSet::from([(0, 0)]));
        let mut check = find_coords(&grid, &syms);
        assert_eq!(check, HashSet::from([(0, 1), (1, 0), (1, 1)]));
        let mut parts = find_parts(&grid, &mut check);
        parts.sort();
        assert_eq!(parts, [2, 100]);
    }

    fn test_example(t: fn(&Vec<String>) -> u64, expected: u64) {
        let lines = const_to_lines(EXAMPLE);
        assert_eq!(t(&lines), expected);
    }

    #[test]
    fn example1() {
        test_example(process1, 4361);
    }

    #[test]
    fn example2() {
        test_example(process2, 467835);
    }
}
