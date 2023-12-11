use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Pipe {
    Vert,
    Horiz,
    BendL,
    BendJ,
    Bend7,
    BendF,
    Empty,
    Start,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Pipe {
    fn is_corner(&self) -> bool {
        match self {
            Pipe::BendL | Pipe::BendJ | Pipe::Bend7 | Pipe::BendF => true,
            _ => false,
        }
    }

    fn is_run_diag(a: Pipe, b: Pipe) -> bool {
        match (a, b) {
            (Pipe::BendF, Pipe::BendJ)
            | (Pipe::BendJ, Pipe::BendF)
            | (Pipe::BendL, Pipe::Bend7)
            | (Pipe::Bend7, Pipe::BendL) => true,
            _ => false,
        }
    }

    fn go(&self, from: Dir) -> Dir {
        let dirs = match self {
            Pipe::Vert => (Dir::Up, Dir::Down),
            Pipe::Horiz => (Dir::Left, Dir::Right),
            Pipe::BendL => (Dir::Up, Dir::Right),
            Pipe::BendJ => (Dir::Up, Dir::Left),
            Pipe::Bend7 => (Dir::Left, Dir::Down),
            Pipe::BendF => (Dir::Down, Dir::Right),
            _ => panic!("invalid travel"),
        };
        if dirs.0 == from.opposite() {
            dirs.1
        } else {
            dirs.0
        }
    }
}

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

fn parse_pipe(c: char) -> Pipe {
    match c {
        '|' => Pipe::Vert,
        '-' => Pipe::Horiz,
        'L' => Pipe::BendL,
        'J' => Pipe::BendJ,
        '7' => Pipe::Bend7,
        'F' => Pipe::BendF,
        '.' => Pipe::Empty,
        'S' => Pipe::Start,
        _ => panic!("unknown pipe"),
    }
}

fn parse(input: &String) -> Vec<Vec<Pipe>> {
    input
        .lines()
        .map(|x| x.chars().map(parse_pipe).collect())
        .collect()
}

fn find_start(map: &Vec<Vec<Pipe>>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == Pipe::Start {
                return (x, y);
            }
        }
    }
    panic!("no start");
}

fn find_start_dir(map: &Vec<Vec<Pipe>>, x: usize, y: usize) -> Dir {
    if y < map.len() - 1
        && (map[y + 1][x] == Pipe::Vert
            || map[y + 1][x] == Pipe::BendL
            || map[y + 1][x] == Pipe::BendJ)
    {
        Dir::Down
    } else if y > 0
        && (map[y - 1][x] == Pipe::Vert
            || map[y - 1][x] == Pipe::Bend7
            || map[y - 1][x] == Pipe::BendF)
    {
        Dir::Up
    } else if x > 0
        && (map[y][x - 1] == Pipe::Horiz
            || map[y][x - 1] == Pipe::BendL
            || map[y][x - 1] == Pipe::BendF)
    {
        Dir::Left
    } else if x < map[0].len() - 1
        && (map[y][x + 1] == Pipe::Horiz
            || map[y][x + 1] == Pipe::BendJ
            || map[y][x + 1] == Pipe::Bend7)
    {
        Dir::Right
    } else {
        panic!("can't get started");
    }
}

fn travel(x: usize, y: usize, dir: Dir) -> (usize, usize) {
    match dir {
        Dir::Up => (x, y - 1),
        Dir::Down => (x, y + 1),
        Dir::Left => (x - 1, y),
        Dir::Right => (x + 1, y),
    }
}

fn run(input: &String) -> (Vec<Vec<Pipe>>, HashSet<(usize, usize)>, i64) {
    let map = parse(input);
    let (mut x, mut y) = find_start(&map);
    let mut dir = find_start_dir(&map, x, y);
    let mut points = HashSet::new();

    points.insert((x, y));
    (x, y) = travel(x, y, dir);
    let mut steps = 1;

    while map[y][x] != Pipe::Start {
        points.insert((x, y));
        dir = map[y][x].go(dir);
        (x, y) = travel(x, y, dir);
        steps += 1;
    }

    (map, points, steps / 2)
}

fn process1(input: &String) -> i64 {
    run(input).2
}

fn process2(input: &String, replace: Pipe) -> i64 {
    let (mut map, points, _) = run(input);
    let (x, y) = find_start(&map);
    map[y][x] = replace;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !points.contains(&(x, y)) {
                map[y][x] = Pipe::Empty;
            }
        }
    }

    let mut count = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] != Pipe::Empty {
                continue;
            }
            let mut cross = 0;
            let mut edge = false;
            let mut corner = Pipe::Empty;
            for x in x..map[0].len() {
                if map[y][x] == Pipe::Vert {
                    cross += 1;
                } else if map[y][x].is_corner() {
                    if !edge {
                        corner = map[y][x];
                    } else if Pipe::is_run_diag(map[y][x], corner) {
                        cross += 1;
                    }
                    edge = !edge;
                }
            }
            if cross % 2 != 0 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    const INPUT: &str = "input.txt";
    let input = read_to_string(INPUT).unwrap();
    let total = process1(&input);
    println!("1: {total}");
    let total = process2(&input, Pipe::Vert); // FIXME hardcoded
    println!("2: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1a() {
        let input = read_to_string("example-a.txt").unwrap();
        assert_eq!(process1(&input), 4);
    }

    #[test]
    fn example1b() {
        let input = read_to_string("example-b.txt").unwrap();
        assert_eq!(process1(&input), 8);
    }

    #[test]
    fn example2c() {
        let input = read_to_string("example-c.txt").unwrap();
        assert_eq!(process2(&input, Pipe::BendF), 4); // FIXME hardcoded
    }

    #[test]
    fn example2d() {
        let input = read_to_string("example-d.txt").unwrap();
        assert_eq!(process2(&input, Pipe::BendF), 8); // FIXME hardcoded
    }

    #[test]
    fn example2e() {
        let input = read_to_string("example-e.txt").unwrap();
        assert_eq!(process2(&input, Pipe::Bend7), 10); // FIXME hardcoded
    }
}
