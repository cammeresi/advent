use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Space {
    Ash,
    Rock,
}

fn parse_space(c: char) -> Space {
    match c {
        '.' => Space::Ash,
        '#' => Space::Rock,
        _ => panic!("unknown space"),
    }
}

fn parse(input: &String) -> Vec<Vec<Vec<Space>>> {
    let mut maps = Vec::new();
    let mut map = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            maps.push(map);
            map = Vec::new();
        } else {
            map.push(line.chars().map(parse_space).collect());
        }
    }

    maps.push(map);
    maps
}

fn find_mirror_row(map: &Vec<Vec<Space>>, not: Option<usize>) -> Option<usize> {
    'middle: for m in 0..map.len() - 1 {
        // after
        let top = m + 1;
        let bottom = map.len() - 1 - m;
        let check = std::cmp::min(top, bottom);
        for i in 0..check {
            let y0 = m - i;
            let y1 = m + 1 + i;
            if map[y0] != map[y1] {
                continue 'middle;
            }
        }
        if not.is_none() || not.unwrap() != m {
            return Some(m);
        }
    }
    None
}

fn find_mirror_rows(maps: &Vec<Vec<Vec<Space>>>) -> Vec<usize> {
    let mut rows = Vec::new();
    for map in maps {
        match find_mirror_row(map, None) {
            None => (),
            Some(m) => rows.push(m),
        };
    }
    rows
}

fn find_mirror_col(map: &Vec<Vec<Space>>, not: Option<usize>) -> Option<usize> {
    'middle: for m in 0..map[0].len() - 1 {
        // after
        let left = m + 1;
        let right = map[0].len() - 1 - m;
        let check = std::cmp::min(left, right);
        for i in 0..check {
            let x0 = m - i;
            let x1 = m + 1 + i;
            for y in 0..map.len() {
                if map[y][x0] != map[y][x1] {
                    continue 'middle;
                }
            }
        }
        if not.is_none() || not.unwrap() != m {
            return Some(m);
        }
    }
    None
}

fn find_mirror_cols(maps: &Vec<Vec<Vec<Space>>>) -> Vec<usize> {
    let mut cols = Vec::new();
    for map in maps {
        match find_mirror_col(map, None) {
            None => (),
            Some(m) => cols.push(m),
        };
    }
    cols
}

fn process1(input: &String) -> usize {
    let maps = parse(input);
    let rows = find_mirror_rows(&maps);
    let cols = find_mirror_cols(&maps);
    cols.iter().map(|x| x + 1).sum::<usize>()
        + rows.iter().map(|x| 100 * (x + 1)).sum::<usize>()
}

fn swap(map: &mut Vec<Vec<Space>>, x: usize, y: usize) {
    map[y][x] = match map[y][x] {
        Space::Ash => Space::Rock,
        Space::Rock => Space::Ash,
    };
}

fn maybe_push(new: Option<usize>, res: &mut Vec<usize>) -> bool {
    match new {
        None => false,
        Some(x) => {
            res.push(x);
            true
        },
    }
}

fn process2(input: &String) -> usize {
    let mut maps = parse(input);
    let mut rows = Vec::new();
    let mut cols = Vec::new();

    'outer: for i in 0..maps.len() {
        assert_eq!(rows.len() + cols.len(), i);
        for y in 0..maps[i].len() {
            for x in 0..maps[i][0].len() {
                let mut map = &mut maps[i];
                let old = find_mirror_row(&map, None);
                swap(&mut map, x, y);
                let new = find_mirror_row(&map, old);
                swap(&mut map, x, y);
                if maybe_push(new, &mut rows) {
                    continue 'outer;
                }

                let old = find_mirror_col(&map, None);
                swap(&mut map, x, y);
                let new = find_mirror_col(&map, old);
                swap(&mut map, x, y);
                if maybe_push(new, &mut cols) {
                    continue 'outer;
                }
            }
        }
    }
    assert_eq!(rows.len() + cols.len(), maps.len());
    cols.iter().map(|x| x + 1).sum::<usize>()
        + rows.iter().map(|x| 100 * (x + 1)).sum::<usize>()
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
        assert_eq!(process1(&input), 405);
    }

    #[test]
    fn example2() {
        let input = read_to_string(EXAMPLE).unwrap();
        assert_eq!(process2(&input), 400);
    }

    #[test]
    fn example3() {
        let input = read_to_string("ex3.txt").unwrap();
        let maps = parse(&input);
        assert_eq!(find_mirror_row(&maps[0], None), Some(0));
    }

    #[test]
    fn example4() {
        let input = read_to_string("ex4.txt").unwrap();
        assert_eq!(process1(&input), 3);
        assert_eq!(process2(&input), 1);
    }

    #[test]
    fn example5() {
        let input = read_to_string("ex5.txt").unwrap();
        assert_eq!(process1(&input), 1);
        assert_eq!(process2(&input), 9);
    }
}
