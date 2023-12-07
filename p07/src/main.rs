use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let mut hands = parse("input");
    hands.sort_by(|a, b| a.compare(b));
    println!("p1: {:?}", part1(&hands));
    hands.sort_by(|a, b| a.compare_with_joker(b));
    println!("p2: {:?}", part1(&hands));
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    cards: String,
    bet: u64
}

const SCORE: &'static str = "23456789TJQKA";
const JSCORE: &'static str = "J23456789TQKA";

impl Hand {
    fn compare(&self, hand: &Hand) -> Ordering {
        let x = self.score(false);
        let y = hand.score(false);

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

    fn compare_with_joker(&self, hand: &Hand) -> Ordering {
        let x = self.score(true);
        let y = hand.score(true);

        if x > y {
            Ordering::Greater
        } else if x < y {
            Ordering::Less
        } else {
            let mut h = hand.cards.chars();
            for c in self.cards.chars() {
                let d = h.next().unwrap();
                let c_score = JSCORE.find(c);
                let d_score = JSCORE.find(d);

                if c_score > d_score {
                    return Ordering::Greater
                } else if c_score < d_score {
                    return Ordering::Less
                }
            }
            panic!("Impossible")
        }
    }

    fn score(&self, jokers: bool) -> u8 {
        let hand_type = self.hand_type(jokers);
        let hand_types = HashMap::from([
            ("five_of_a_kind", 7),
            ("four_of_a_kind", 6),
            ("full_house", 5),
            ("three_of_a_kind", 4),
            ("two_pair", 3),
            ("one_pair", 2),
            ("high_card", 1)
        ]);
        hand_types[hand_type]
    }

    fn hand_type(&self, jokers: bool) -> &str {
        let mut map = HashMap::new();
        for c in self.cards.chars() {
            let e = map.entry(c).or_insert(0);
            *e += 1
        }

        if jokers {
            let max_k = max_k_v(&map);
            let js = get_joker_count(&map);
            if let Some(n) = map.get_mut(&max_k) {
                *n += js
            }
            map.remove(&'J');
        }

        let mut count_map = HashMap::new();
        for (_, v) in &map {
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
        } else {
            return "five_of_a_kind"
        }
    }
}

fn get_joker_count(map: &HashMap<char, u64>) -> u64 {
    return *map.get(&'J').unwrap_or(&0)
}

fn max_k_v(map: &HashMap<char, u64>) -> char {
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


#[test]
fn test_card_type() {
    let high_card = Hand { cards: String::from("23456"), bet: 0};
    assert_eq!(high_card.hand_type(false), "high_card");
    let one_pair = Hand { cards: String::from("A23A4"), bet: 0};
    assert_eq!(one_pair.hand_type(false), "one_pair");
    let two_pair = Hand { cards: String::from("23432"), bet: 0};
    assert_eq!(two_pair.hand_type(false), "two_pair");
    let three_oak = Hand { cards: String::from("FFF89"), bet: 0};
    assert_eq!(three_oak.hand_type(false), "three_of_a_kind");
    let full_house = Hand { cards: String::from("FFF99"), bet: 0};
    assert_eq!(full_house.hand_type(false), "full_house");
    let four_oak = Hand { cards: String::from("FFFF9"), bet: 0};
    assert_eq!(four_oak.hand_type(false), "four_of_a_kind");
    let five_oak = Hand { cards: String::from("FFFFF"), bet: 0};
    assert_eq!(five_oak.hand_type(false), "five_of_a_kind");
}

#[test]
fn test_card_type_with_joker() {
    let high_card = Hand { cards: String::from("23456"), bet: 0};
    assert_eq!(high_card.hand_type(true), "high_card");
    let one_pair = Hand { cards: String::from("A23A4"), bet: 0};
    assert_eq!(one_pair.hand_type(true), "one_pair");
    let two_pair = Hand { cards: String::from("23432"), bet: 0};
    assert_eq!(two_pair.hand_type(true), "two_pair");
    let three_oak = Hand { cards: String::from("FFF89"), bet: 0};
    assert_eq!(three_oak.hand_type(true), "three_of_a_kind");
    let full_house = Hand { cards: String::from("FFF99"), bet: 0};
    assert_eq!(full_house.hand_type(true), "full_house");
    let four_oak = Hand { cards: String::from("FFFF9"), bet: 0};
    assert_eq!(four_oak.hand_type(true), "four_of_a_kind");
    let five_oak = Hand { cards: String::from("FFFFF"), bet: 0};
    assert_eq!(five_oak.hand_type(true), "five_of_a_kind");
    let j_one_pair = Hand { cards: String::from("A23J4"), bet: 0};
    assert_eq!(j_one_pair.hand_type(true), "one_pair");
    let j_two_pair = Hand { cards: String::from("2J432"), bet: 0};
    assert_eq!(j_two_pair.hand_type(true), "three_of_a_kind");
    let j_three_oak = Hand { cards: String::from("FJJ89"), bet: 0};
    assert_eq!(j_three_oak.hand_type(true), "three_of_a_kind");
    let j_full_house = Hand { cards: String::from("FFFJJ"), bet: 0};
    assert_eq!(j_full_house.hand_type(true), "five_of_a_kind");
    let j_four_oak = Hand { cards: String::from("JJJ98"), bet: 0};
    assert_eq!(j_four_oak.hand_type(true), "four_of_a_kind");
    let j_five_oak = Hand { cards: String::from("JJJJJ"), bet: 0};
    assert_eq!(j_five_oak.hand_type(true), "five_of_a_kind");
}

#[test]
fn test_joker_card() {
    let a = Hand { cards: String::from("JJJJJ"), bet: 0};
    assert_eq!(a.score(true), 7);
    let b = Hand { cards: String::from("8JJJJ"), bet: 0};
    assert_eq!(b.score(true), 7);
    let c = Hand { cards: String::from("87JJJ"), bet: 0};
    assert_eq!(c.score(true), 6);
    let d = Hand { cards: String::from("88JJJ"), bet: 0};
    assert_eq!(d.score(true), 7);
    let e = Hand { cards: String::from("888JJ"), bet: 0};
    assert_eq!(e.score(true), 7);
    let f = Hand { cards: String::from("8888J"), bet: 0};
    assert_eq!(f.score(true), 7);
    let g = Hand { cards: String::from("1234J"), bet: 0};
    assert_eq!(g.score(true), 2);
    let h = Hand { cards: String::from("1244J"), bet: 0};
    assert_eq!(h.score(true), 4);
    let i = Hand { cards: String::from("1444J"), bet: 0};
    assert_eq!(i.score(true), 6);
    let j = Hand { cards: String::from("123JJ"), bet: 0};
    assert_eq!(j.score(true), 4);
    let k = Hand { cards: String::from("122JJ"), bet: 0};
    assert_eq!(k.score(true), 6);
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

    assert_eq!(part1(&hands), 6440);

    hands.sort_by(|a, b| a.compare_with_joker(b));
    for h in &hands {
        println!("{:?}", h);
    }
    assert_eq!(part1(&hands), 5905);
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
