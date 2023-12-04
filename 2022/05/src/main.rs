use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Stacks = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    num: usize,
    from: usize,
    to: usize,
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

fn parse_stacks(lines: &[String]) -> Stacks {
    let numbers = &lines[lines.len() - 1];
    let lines = lines
        .iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let num = numbers.split_whitespace().count();
    let mut stacks = Vec::new();

    for _s in 0..num {
        stacks.push(Vec::new());
    }
    for s in 0..num {
        for h in (0..lines.len() - 1).rev() {
            let c = lines[h][4 * s + 1];
            if c != ' ' {
                stacks[s].push(c);
            }
        }
    }
    stacks
}

fn parse_moves(lines: &[String]) -> Vec<Move> {
    let lines = lines
        .iter()
        .map(|x| x.split(" ").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut moves = Vec::new();
    for m in lines {
        moves.push(Move {
            num: m[1].parse().unwrap(),
            from: m[3].parse::<usize>().unwrap() - 1,
            to: m[5].parse::<usize>().unwrap() - 1,
        })
    }
    moves
}

fn split_input(lines: &mut Vec<String>) -> (&[String], &[String]) {
    let split = lines.iter_mut().position(|x| x == "").unwrap();
    (&lines[..split], &lines[split + 1..])
}

fn parse_input(lines: &mut Vec<String>) -> (Stacks, Vec<Move>) {
    let (stacks, moves) = split_input(lines);
    (parse_stacks(&stacks), parse_moves(&moves))
}

fn execute_moves(stacks: &mut Stacks, moves: Vec<Move>, advanced: bool) {
    for m in moves {
        if !advanced {
            for _i in 0..m.num {
                let x = stacks[m.from].pop().unwrap();
                stacks[m.to].push(x);
            }
        } else {
            let mut tmp = Vec::new();
            for _i in 0..m.num {
                let x = stacks[m.from].pop().unwrap();
                tmp.push(x);
            }
            for _i in 0..m.num {
                let x = tmp.pop().unwrap();
                stacks[m.to].push(x);
            }
        }
    }
}

fn get_tops(stacks: &Stacks) -> String {
    let mut tops = String::new();
    for s in stacks {
        if let Some(c) = s.last() {
            tops.push(*c);
        }
    }
    tops
}

fn process1(lines: &mut Vec<String>) -> String {
    let (mut stacks, moves) = parse_input(lines);
    execute_moves(&mut stacks, moves, false);
    get_tops(&stacks)
}

fn process2(lines: &mut Vec<String>) -> String {
    let (mut stacks, moves) = parse_input(lines);
    execute_moves(&mut stacks, moves, true);
    get_tops(&stacks)
}

fn main() {
    const INPUT: &str = "input.txt";
    let mut lines = read_lines(INPUT);
    let total = process1(&mut lines);
    println!("1: {total}");
    let total = process2(&mut lines);
    println!("2: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
        "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\
        \n\
        move 1 from 2 to 1\n\
        move 3 from 1 to 3\n\
        move 2 from 2 to 1\n\
        move 1 from 1 to 2";

    fn const_to_lines(c: &str) -> Vec<String> {
        c.lines().map(|x| String::from(x)).collect()
    }

    #[test]
    fn test_simple() {
        let mut lines = const_to_lines(EXAMPLE);
        let (stacks, moves) = split_input(&mut lines);
        assert_eq!(stacks.len(), 4);
        assert_eq!(moves.len(), 4);
        let stacks = parse_stacks(stacks);
        assert_eq!(
            stacks,
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
        );
        let moves = parse_moves(moves);
        assert_eq!(
            moves[0],
            Move {
                num: 1,
                from: 1,
                to: 0,
            }
        );
    }

    fn test_example(t: fn(&mut Vec<String>) -> String, expected: &str) {
        let mut lines = const_to_lines(EXAMPLE);
        assert_eq!(t(&mut lines), expected.to_string());
    }

    #[test]
    fn example1() {
        test_example(process1, "CMZ");
    }

    #[test]
    fn example2() {
        test_example(process2, "MCD");
    }
}
