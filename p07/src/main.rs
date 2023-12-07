use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let mut hands = parse("input");
    hands.sort_by(|a, b| a.compare(b, false));
    println!("p1: {:?}", part1(&hands));
    hands.sort_by(|a, b| a.compare(b, true));
    println!("p2: {:?}", part1(&hands));
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    cards: String,
    bet: u64
}

const SCORE: &str = "23456789TJQKA";
const JSCORE: &str = "J23456789TQKA";

impl Hand {
    fn compare(&self, hand: &Hand, jokers: bool) -> Ordering {
        let score = if jokers { JSCORE } else { SCORE };
        let c = self.score(jokers).cmp(&hand.score(jokers));

        if c != Ordering::Equal {
            return c
        }

        let mut h = hand.cards.chars();

        for c in self.cards.chars() {
            let d = h.next().unwrap();
            let cmp = score.find(c).cmp(&score.find(d));

            if cmp != Ordering::Equal {
                return cmp
            }
        }

        panic!("Impossible")
    }

    fn score(&self, jokers: bool) -> u8 {
        let mut map = HashMap::new();

        for c in self.cards.chars() {
            let e = map.entry(c).or_insert(0);
            *e += 1
        }

        if jokers {
            let max_k = highest_non_joker_card(&map);
            let js = *map.get(&'J').unwrap_or(&0);
            if let Some(n) = map.get_mut(&max_k) {
                *n += js
            }
            map.remove(&'J');
        }

        let mut v: Vec<&u64> = map.values().collect();
        v.sort();

        let signature: String = v
            .iter()
            .map(|n| char::from_digit(**n as u32, 10).unwrap())
            .collect();

        match signature.as_str() {
            "5"     => 7,
            "14"    => 6,
            "23"    => 5,
            "113"   => 4,
            "122"   => 3,
            "1112"  => 2,
            "11111" => 1,
            ""      => 7,
            _       => 0
        }
    }
}

fn highest_non_joker_card(map: &HashMap<char, u64>) -> char {
    let mut max_k = &' ';
    let mut max_v = &0;
    for (k, v) in map {
        if v > max_v && k != &'J' {
            max_v = v;
            max_k = k;
        }
    }
    *max_k
}

fn part1(hands: &[Hand]) -> u64 {
    hands
        .iter()
        .enumerate().map(|(i, h)| h.bet * (i + 1) as u64)
        .sum()
}

fn parse(input: &'static str) -> Vec<Hand> {
    let mut hands = vec![];
    let file = File::open(input).unwrap();
    let lines = BufReader::new(file).lines();

    for line in lines {
        let l = line.unwrap();
        let (hand, bet) = l.split_once(' ').unwrap();

        hands.push(
            Hand {
                cards: String::from(hand),
                bet: bet.parse::<u64>().unwrap()
            }
        )
    }

    hands
}

#[test]
fn test_compare_card() {
    let hcardx = Hand { cards: String::from("23456"), bet: 0};
    let hcardy = Hand { cards: String::from("73456"), bet: 0};
    assert_eq!(hcardx.compare(&hcardy, false), Ordering::Less);

    let fcardx = Hand { cards: String::from("33332"), bet: 0};
    let fcardy = Hand { cards: String::from("2AAAA"), bet: 0};
    assert_eq!(fcardx.compare(&fcardy, false), Ordering::Greater);

    let tcardx = Hand { cards: String::from("KK677"), bet: 0};
    let tcardy = Hand { cards: String::from("KTJJT"), bet: 0};
    assert_eq!(tcardx.compare(&tcardy, false), Ordering::Greater);

    let ucardx = Hand { cards: String::from("KK677"), bet: 0};
    let ucardy = Hand { cards: String::from("KTJJT"), bet: 0};
    assert_eq!(ucardx.compare(&ucardy, false), Ordering::Greater);

    let vcardx = Hand { cards: String::from("T55J5"), bet: 0};
    let vcardy = Hand { cards: String::from("QQQJA"), bet: 0};
    assert_eq!(vcardx.compare(&vcardy, false), Ordering::Less)
}

#[test]
fn test_part1() {
    let mut hands = parse("test_input");
    hands.sort_by(|a, b| a.compare(b, false));

    assert_eq!(part1(&hands), 6440);

    hands.sort_by(|a, b| a.compare(b, true));
    for h in &hands {
        println!("{:?}", h);
    }
    assert_eq!(part1(&hands), 5905);
}

#[test]
fn test_card_type_with_joker() {
    let high_card = Hand { cards: String::from("23456"), bet: 0};
    assert_eq!(high_card.score(true), 1);
    let one_pair = Hand { cards: String::from("A23A4"), bet: 0};
    assert_eq!(one_pair.score(true), 2);
    let two_pair = Hand { cards: String::from("23432"), bet: 0};
    assert_eq!(two_pair.score(true), 3);
    let three_oak = Hand { cards: String::from("FFF89"), bet: 0};
    assert_eq!(three_oak.score(true), 4);
    let full_house = Hand { cards: String::from("FFF99"), bet: 0};
    assert_eq!(full_house.score(true), 5);
    let four_oak = Hand { cards: String::from("FFFF9"), bet: 0};
    assert_eq!(four_oak.score(true), 6);
    let five_oak = Hand { cards: String::from("FFFFF"), bet: 0};
    assert_eq!(five_oak.score(true), 7);
    let j_one_pair = Hand { cards: String::from("A23J4"), bet: 0};
    assert_eq!(j_one_pair.score(true), 2);
    let j_two_pair = Hand { cards: String::from("2J432"), bet: 0};
    assert_eq!(j_two_pair.score(true), 4);
    let j_three_oak = Hand { cards: String::from("FJJ89"), bet: 0};
    assert_eq!(j_three_oak.score(true), 4);
    let j_full_house = Hand { cards: String::from("FFFJJ"), bet: 0};
    assert_eq!(j_full_house.score(true), 7);
    let j_four_oak = Hand { cards: String::from("JJJ98"), bet: 0};
    assert_eq!(j_four_oak.score(true), 6);
    let j_five_oak = Hand { cards: String::from("JJJJJ"), bet: 0};
    assert_eq!(j_five_oak.score(true), 7);
}

#[test]
fn test_card_type() {
    let high_card = Hand { cards: String::from("23456"), bet: 0};
    assert_eq!(high_card.score(false), 1);
    let one_pair = Hand { cards: String::from("A23A4"), bet: 0};
    assert_eq!(one_pair.score(false), 2);
    let two_pair = Hand { cards: String::from("23432"), bet: 0};
    assert_eq!(two_pair.score(false), 3);
    let three_oak = Hand { cards: String::from("FFF89"), bet: 0};
    assert_eq!(three_oak.score(false), 4);
    let full_house = Hand { cards: String::from("FFF99"), bet: 0};
    assert_eq!(full_house.score(false), 5);
    let four_oak = Hand { cards: String::from("FFFF9"), bet: 0};
    assert_eq!(four_oak.score(false), 6);
    let five_oak = Hand { cards: String::from("FFFFF"), bet: 0};
    assert_eq!(five_oak.score(false), 7);
}
