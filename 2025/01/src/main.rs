use std::fs::read_to_string;

enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("not direction"),
        }
    }
}

fn run(input: &String, multi: bool) -> usize {
    let mut pos: isize = 50;
    let mut zeroes = 0;
    for line in input.lines() {
        let dir = Direction::from(line.chars().next().expect("no dir"));
        let num: isize = line[1..].parse().expect("num");

        for _ in 0..num {
            match dir {
                Direction::Left => pos -= 1,
                Direction::Right => pos += 1,
            }

            if pos < 0 {
                pos += 100;
            } else if pos >= 100 {
                pos -= 100;
            }

            if multi && pos == 0 {
                zeroes += 1;
            }
        }

        if !multi && pos == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

fn process1(input: &String) -> usize {
    run(input, false)
}

fn process2(input: &String) -> usize {
    run(input, true)
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
    fn example0() {
        let input = read_to_string(EXAMPLE).unwrap();
        assert_eq!(process1(&input), 3);
    }

    #[test]
    fn example1() {
        let input = read_to_string(EXAMPLE).unwrap();
        assert_eq!(process2(&input), 6);
    }
}
