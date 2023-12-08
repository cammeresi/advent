use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::read_to_string;

const CARDS: usize = 5;

#[derive(Debug, Copy, Clone, Eq)]
struct Hand {
    cards: [u8; CARDS],
    wilds: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Game {
    hand: Hand,
    bid: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl Game {
    fn new(hand: &str, bid: usize) -> Game {
        let mut g = Game {
            hand: Hand { cards: [0; CARDS], wilds: 0, },
            bid: bid,
        };
        assert_eq!(hand.len(), CARDS);
        let hand = hand.chars().collect::<Vec<_>>();
        for i in 0..CARDS {
            g.hand.cards[i] = match hand[i] {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '9' => 9,
                '8' => 8,
                '7' => 7,
                '6' => 6,
                '5' => 5,
                '4' => 4,
                '3' => 3,
                '2' => 2,
                _ => panic!("unknown card"),
            };
        }
        g
    }
}

impl Hand {
    // only for hand type; sorted low to high, unlike poker
    fn canonicalize(&self) -> [u8; CARDS] {
        let mut res = self.cards;
        res.sort();
        res
    }

    fn hand_has(&self, card: u8, count: usize) -> bool {
        card != 1 && self.cards.iter().filter(|x| **x == card).count() == count
    }

    fn hand_is(&self, count: usize, a: &[u8; CARDS]) -> bool {
        for i in 0..CARDS {
            if self.hand_has(a[i], count) {
                return true;
            }
        }
        false
    }

    fn hand_is_five(&self, a: &[u8; CARDS]) -> bool {
        self.hand_is(5, a)
    }

    fn hand_is_four(&self, a: &[u8; CARDS]) -> bool {
        self.hand_is(4, a)
    }

    fn hand_is_full(&self, a: &[u8; CARDS]) -> bool {
        self.hand_is_three(a) && self.hand_is_pair(a)
    }

    fn hand_is_three(&self, a: &[u8; CARDS]) -> bool {
        self.hand_is(3, a)
    }

    fn hand_is_pair(&self, a: &[u8; CARDS]) -> bool {
        for i in 0..CARDS - 1 {
            if self.hand_has(a[i], 2) {
                return true;
            }
        }
        false
    }

    fn hand_is_two_pair(&self, a: &[u8; CARDS]) -> bool {
        let mut pairs = 0;
        for i in 0..CARDS - 1 {
            if a[i] == a[i + 1] && a[i] != 1 {
                pairs += 1;
            }
        }
        pairs == 2
    }

    fn hand_type(&self) -> HandType {
        let a = self.canonicalize();
        let mut t = if self.hand_is_five(&a) {
            HandType::FiveKind
        } else if self.hand_is_four(&a) {
            HandType::FourKind
        } else if self.hand_is_full(&a) {
            HandType::FullHouse
        } else if self.hand_is_three(&a) {
            HandType::ThreeKind
        } else if self.hand_is_two_pair(&a) {
            HandType::TwoPair
        } else if self.hand_is_pair(&a) {
            HandType::OnePair
        } else {
            HandType::HighCard
        };
        for _ in 0..self.wilds {
            t = match t {
                HandType::HighCard => HandType::OnePair,
                HandType::OnePair => HandType::ThreeKind,
                HandType::TwoPair => HandType::FullHouse,
                HandType::ThreeKind => HandType::FourKind,
                _ => HandType::FiveKind,
            };
        }
        t
    }

    fn wild(&mut self) {
        for i in 0..CARDS {
            if self.cards[i] == 11 {
                self.wilds += 1;
                self.cards[i] = 1;
            }
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let t = self.hand_type().cmp(&other.hand_type());
        if t != Ordering::Equal {
            t
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

fn parse(input: &String) -> Vec<Game> {
    let mut games = Vec::new();

    for line in input.lines() {
        let (hand, bid) = line.split_whitespace().collect_tuple().unwrap();
        let bid = bid.parse::<usize>().unwrap();
        games.push(Game::new(hand, bid));
    }

    games
}

fn process1(input: &String) -> usize {
    let mut games = parse(input);
    games.sort();
    games.iter().enumerate().fold(0, |x, (i, g)| x + (i + 1) * g.bid)
}

fn process2(input: &String) -> usize {
    let mut games = parse(input);
    for g in games.iter_mut() {
        g.hand.wild();
    }
    games.sort();
    games.iter().enumerate().fold(0, |x, (i, g)| x + (i + 1) * g.bid)
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

    fn test_example(t: fn(&String) -> usize, expected: usize) {
        let input = super::read_to_string(INPUT).unwrap();
        assert_eq!(t(&input), expected);
    }

    #[test]
    fn test_simple() {
        let a = Hand {
            cards: [7, 7, 7, 7, 7], wilds: 0,
        };
        assert_eq!(a.hand_type(), HandType::FiveKind);
        let a = Hand {
            cards: [7, 7, 6, 7, 7], wilds: 0,
        };
        assert_eq!(a.hand_type(), HandType::FourKind);
        let a = Hand {
            cards: [7, 7, 6, 7, 6], wilds: 0,
        };
        assert_eq!(a.hand_type(), HandType::FullHouse);
        let a = Hand {
            cards: [7, 7, 6, 7, 5], wilds: 0,
        };
        assert_eq!(a.hand_type(), HandType::ThreeKind);
        let a = Hand {
            cards: [7, 7, 6, 4, 5], wilds: 0,
        };
        assert_eq!(a.hand_type(), HandType::OnePair);
        let a = Hand {
            cards: [7, 7, 6, 6, 5], wilds: 0,
        };
        assert_eq!(a.hand_type(), HandType::TwoPair);
        let a = Hand {
            cards: [8, 7, 6, 5, 4], wilds: 0,
        };
        assert_eq!(a.hand_type(), HandType::HighCard);

        let x = Hand {
            cards: [7, 7, 6, 13, 13], wilds: 0,
        };
        let y = Hand {
            cards: [13, 10, 11, 11, 10], wilds: 0,
        };
        assert!(x < y);
    }

    #[test]
    fn test_wilds() {
        let mut x = Hand {
            cards: [7, 7, 6, 7, 11], wilds: 0,
        };
        x.wild();
        assert_eq!(x.cards, [7, 7, 6, 7, 1]);
        assert_eq!(x.wilds, 1);
        assert_eq!(x.hand_type(), HandType::FourKind);
        let mut x = Hand {
            cards: [11, 11, 11, 11, 11], wilds: 0,
        };
        x.wild();
        assert_eq!(x.hand_type(), HandType::FiveKind);
        let mut x = Hand {
            cards: [10, 13, 11, 13, 7], wilds: 0,
        };
        x.wild();
        assert_eq!(x.hand_type(), HandType::ThreeKind);
        let x = Hand {
            cards: [14, 12, 12, 1, 1], wilds: 2,
        };
        assert_eq!(x.hand_type(), HandType::FourKind);
    }

    #[test]
    fn example1() {
        test_example(process1, 6440);
    }

    #[test]
    fn example2() {
        test_example(process2, 5905);
    }
}
