use std::fs::read_to_string;

fn parse(input: &String) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|x| x.split_whitespace().map(|y| y.parse().unwrap()).collect())
        .collect()
}

fn extrapolate(m: &Vec<i64>, rev: bool) -> i64 {
    let mut seqs = Vec::new();
    let mut m = m.clone();
    if rev {
        m.reverse();
    }
    seqs.push(m);

    while !seqs.last().unwrap().iter().all(|x| *x == 0) {
        let mut s = Vec::new();
        let last = seqs.last().unwrap();
        for i in 0..last.len() - 1 {
            s.push(last[i + 1] - last[i]);
        }
        seqs.push(s);
    }

    let mut new = Vec::new();
    new.push(0);
    for i in (1..seqs.len()).rev() {
        let a = &seqs[i - 1];
        new.push(new[new.len() - 1] + a[a.len() - 1]);
    }

    new[new.len() - 1]
}

fn process1(input: &String) -> i64 {
    let metrics = parse(input);
    metrics.iter().map(|x| extrapolate(x, false)).sum()
}

fn process2(input: &String) -> i64 {
    let metrics = parse(input);
    metrics.iter().map(|x| extrapolate(x, true)).sum()
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

    const INPUT: &str = "example.txt";

    fn test_example(t: fn(&String) -> i64, expected: i64) {
        let input = super::read_to_string(INPUT).unwrap();
        assert_eq!(t(&input), expected);
    }

    #[test]
    fn example1() {
        test_example(process1, 114);
    }

    #[test]
    fn example2() {
        test_example(process2, 2);
    }
}
