use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let mut hands = parse("input");
    hands.sort_by(|a, b| a.compare(b));

    println!("p1: {:?}", part1(&hands));
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    cards: String,
    bet: u64
}

const SCORE: &'static str = "23456789TJQKA";

impl Hand {
    fn compare(&self, hand: &Hand) -> Ordering {
        let x = self.score();
        let y = hand.score();
        if x > y {
            Ordering::Greater
        } else if x < y {
            Ordering::Less
        } else {
            let mut h = hand.cards.chars();
            for c in self.cards.chars() {
                let d = h.next().unwrap();
                let c_score = SCORE.find(c);
                let d_score = SCORE.find(d);

                if c_score > d_score {
                    return Ordering::Greater
                } else if c_score < d_score {
                    return Ordering::Less
                }
            }
            panic!("Impossible")
        }
    }

    fn score(&self) -> u8 {
        let hand_type = self.hand_type();
        let score = HashMap::from([
            ("five_of_a_kind", 7),
            ("four_of_a_kind", 6),
            ("full_house", 5),
            ("three_of_a_kind", 4),
            ("two_pair", 3),
            ("one_pair", 2),
            ("high_card", 1)
        ]);

        score[hand_type]
    }

    fn hand_type(&self) -> &str {
        let mut map = HashMap::new();
        let mut count_map = HashMap::new();

        for c in self.cards.chars() {
            let e = map.entry(c).or_insert(0);
            *e += 1
        }

        for (_, v) in map {
            let e = count_map.entry(v).or_insert(0);
            *e += 1;
        }

        if count_map.get(&5) == Some(&1) {
            return "five_of_a_kind"
        } else if count_map.get(&4) == Some(&1) {
            return "four_of_a_kind"
        } else if count_map.get(&3) == Some(&1) &&
                  count_map.get(&2) == Some(&1) {
            return "full_house"
        } else if count_map.get(&3) == Some(&1) {
            return "three_of_a_kind"
        } else if count_map.get(&2) == Some(&2) {
            return "two_pair"
        } else if count_map.get(&2) == Some(&1) {
            return "one_pair"
        } else if count_map.get(&1) == Some(&5) {
            return "high_card"
        }

        panic!("invalid")
    }
}

#[test]
fn test_card_type() {
    let high_card = Hand { cards: String::from("23456"), bet: 0};
    assert_eq!(high_card.hand_type(), "high_card");
    let one_pair = Hand { cards: String::from("A23A4"), bet: 0};
    assert_eq!(one_pair.hand_type(), "one_pair");
    let two_pair = Hand { cards: String::from("23432"), bet: 0};
    assert_eq!(two_pair.hand_type(), "two_pair");
    let three_oak = Hand { cards: String::from("FFF89"), bet: 0};
    assert_eq!(three_oak.hand_type(), "three_of_a_kind");
    let full_house = Hand { cards: String::from("FFF99"), bet: 0};
    assert_eq!(full_house.hand_type(), "full_house");
    let four_oak = Hand { cards: String::from("FFFF9"), bet: 0};
    assert_eq!(four_oak.hand_type(), "four_of_a_kind");
    let five_oak = Hand { cards: String::from("FFFFF"), bet: 0};
    assert_eq!(five_oak.hand_type(), "five_of_a_kind");
}

#[test]
fn test_compare_card() {
    let hcardx = Hand { cards: String::from("23456"), bet: 0};
    let hcardy = Hand { cards: String::from("73456"), bet: 0};
    assert_eq!(hcardx.compare(&hcardy), Ordering::Less);

    let fcardx = Hand { cards: String::from("33332"), bet: 0};
    let fcardy = Hand { cards: String::from("2AAAA"), bet: 0};
    assert_eq!(fcardx.compare(&fcardy), Ordering::Greater);

    let tcardx = Hand { cards: String::from("KK677"), bet: 0};
    let tcardy = Hand { cards: String::from("KTJJT"), bet: 0};
    assert_eq!(tcardx.compare(&tcardy), Ordering::Greater);

    let ucardx = Hand { cards: String::from("KK677"), bet: 0};
    let ucardy = Hand { cards: String::from("KTJJT"), bet: 0};
    assert_eq!(ucardx.compare(&ucardy), Ordering::Greater);

    let vcardx = Hand { cards: String::from("T55J5"), bet: 0};
    let vcardy = Hand { cards: String::from("QQQJA"), bet: 0};
    assert_eq!(vcardx.compare(&vcardy), Ordering::Less)
}

fn part1(hands: &[Hand]) -> u64 {
    hands
        .iter()
        .enumerate().map(|(i, h)| h.bet * (i + 1) as u64)
        .sum()
}

#[test]
fn test_part1() {
    let mut hands = parse("test_input");
    hands.sort_by(|a, b| a.compare(b));

    assert_eq!(part1(&hands), 6440)
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
