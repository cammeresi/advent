use std::fs::read_to_string;

fn hash_step(input: &str) -> usize {
    input.bytes().fold(0, |x, y| {
        ((((x as u16) + (y as u16)) * 17) % 256).try_into().unwrap()
    })
}

fn process1(input: &String) -> usize {
    input.trim().split(",").map(hash_step).sum()
}

fn process2(input: &String) -> usize {
    let input = input.trim().split(",");
    let mut boxes: Vec<Vec<(&str, usize)>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    'outer: for cmd in input {
        if cmd.contains("=") {
            let mut cmd = cmd.split("=");
            let k = cmd.next().unwrap();
            let v = cmd.next().unwrap().parse::<usize>().unwrap();
            let h = hash_step(k);
            for lens in &mut boxes[h] {
                if lens.0 == k {
                    lens.1 = v;
                    continue 'outer;
                }
            }
            boxes[h].push((k, v));
        } else if cmd.contains("-") {
            let k = cmd.strip_suffix("-").unwrap();
            let h = hash_step(k);
            for i in 0..boxes[h].len() {
                if boxes[h][i].0 == k {
                    boxes[h].remove(i);
                    break;
                }
            }
        } else {
            panic!("bad input");
        }
    }

    let mut total = 0;
    for i in 0..boxes.len() {
        for j in 0..boxes[i].len() {
            total += (i + 1) * (j + 1) * boxes[i][j].1;
        }
    }
    total
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
        assert_eq!(hash_step("rn=1"), 30);
    }

    #[test]
    fn example1() {
        let input = read_to_string(EXAMPLE).unwrap();
        assert_eq!(process1(&input), 1320);
    }

    #[test]
    fn example2() {
        let input = read_to_string(EXAMPLE).unwrap();
        assert_eq!(process2(&input), 145);
    }
}
