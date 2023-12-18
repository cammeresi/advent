use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Space {
    Empty,
    Square,
    Round,
}

fn parse_space(c: char) -> Space {
    match c {
        '.' => Space::Empty,
        '#' => Space::Square,
        'O' => Space::Round,
        _ => panic!("unknown space '{c}'"),
    }
}

fn parse(input: &String) -> Vec<Vec<Space>> {
    let mut map = Vec::new();

    for line in input.lines() {
        map.push(line.chars().map(parse_space).collect());
    }

    map
}

fn roll_north(map: &mut Vec<Vec<Space>>) {
    for y in 1..map.len() {
        for x in 0..map[0].len() {
            let mut y = y;
            while y >= 1
                && map[y][x] == Space::Round
                && map[y - 1][x] == Space::Empty
            {
                map[y][x] = Space::Empty;
                map[y - 1][x] = Space::Round;
                y -= 1;
            }
        }
    }
}

fn roll_south(map: &mut Vec<Vec<Space>>) {
    for y in (0..map.len() - 1).rev() {
        for x in 0..map[0].len() {
            let mut y = y;
            while y <= map.len() - 2
                && map[y][x] == Space::Round
                && map[y + 1][x] == Space::Empty
            {
                map[y][x] = Space::Empty;
                map[y + 1][x] = Space::Round;
                y += 1;
            }
        }
    }
}

fn roll_west(map: &mut Vec<Vec<Space>>) {
    for y in 0..map.len() {
        for x in 1..map[0].len() {
            let mut x = x;
            while x >= 1
                && map[y][x] == Space::Round
                && map[y][x - 1] == Space::Empty
            {
                map[y][x] = Space::Empty;
                map[y][x - 1] = Space::Round;
                x -= 1;
            }
        }
    }
}

fn roll_east(map: &mut Vec<Vec<Space>>) {
    for y in 0..map.len() {
        for x in (0..map[0].len() - 1).rev() {
            let mut x = x;
            while x <= map[0].len() - 2
                && map[y][x] == Space::Round
                && map[y][x + 1] == Space::Empty
            {
                map[y][x] = Space::Empty;
                map[y][x + 1] = Space::Round;
                x += 1;
            }
        }
    }
}

fn spin(map: &mut Vec<Vec<Space>>) {
    roll_north(map);
    roll_west(map);
    roll_south(map);
    roll_east(map);
}

fn load(map: &Vec<Vec<Space>>) -> usize {
    let mut total = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == Space::Round {
                total += map.len() - y;
            }
        }
    }
    total
}

fn process1(input: &String) -> usize {
    let mut map = parse(input);
    roll_north(&mut map);
    load(&map)
}

fn process2(input: &String) -> usize {
    const SPINS: usize = 1000000000;
    let mut map = parse(input);
    let mut seen = HashMap::new();
    let (mut a, mut b) = (None, None);
    for i in 0..SPINS {
        let mut h = DefaultHasher::new();
        map.hash(&mut h);
        let h = h.finish();
        if !seen.contains_key(&h) {
            seen.insert(h, i);
        } else {
            (a, b) = (seen.get(&h), Some(i));
            break;
        }
        spin(&mut map);
    }

    let (a, b) = (a.unwrap(), b.unwrap());
    let len = b - a;
    let restart = (SPINS - a) / len * len + a;
    for _ in restart..SPINS {
        spin(&mut map);
    }
    load(&map)
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
    fn example_north() {
        let input = read_to_string(EXAMPLE).unwrap();
        let mut map = parse(&input);
        roll_north(&mut map);
        let input = read_to_string("example-north.txt").unwrap();
        let expected = parse(&input);
        assert_eq!(map, expected);
    }

    #[test]
    fn example_south() {
        let input = read_to_string(EXAMPLE).unwrap();
        let mut map = parse(&input);
        roll_south(&mut map);
        let input = read_to_string("example-south.txt").unwrap();
        let expected = parse(&input);
        assert_eq!(map, expected);
    }

    #[test]
    fn example_west() {
        let input = read_to_string(EXAMPLE).unwrap();
        let mut map = parse(&input);
        roll_west(&mut map);
        let input = read_to_string("example-west.txt").unwrap();
        let expected = parse(&input);
        assert_eq!(map, expected);
    }

    #[test]
    fn example_east() {
        let input = read_to_string(EXAMPLE).unwrap();
        let mut map = parse(&input);
        roll_east(&mut map);
        let input = read_to_string("example-east.txt").unwrap();
        let expected = parse(&input);
        assert_eq!(map, expected);
    }

    #[test]
    fn example_spin() {
        let input = read_to_string(EXAMPLE).unwrap();
        let mut map = parse(&input);
        spin(&mut map);
        let input = read_to_string("cycle1.txt").unwrap();
        let expected = parse(&input);
        assert_eq!(map, expected);
        spin(&mut map);
        let input = read_to_string("cycle2.txt").unwrap();
        let expected = parse(&input);
        assert_eq!(map, expected);
        spin(&mut map);
        let input = read_to_string("cycle3.txt").unwrap();
        let expected = parse(&input);
        assert_eq!(map, expected);
    }

    #[test]
    fn example1() {
        let input = read_to_string(EXAMPLE).unwrap();
        assert_eq!(process1(&input), 136);
    }

    #[test]
    fn example2() {
        let input = read_to_string(EXAMPLE).unwrap();
        assert_eq!(process2(&input), 64);
    }
}
