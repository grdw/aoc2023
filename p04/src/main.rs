use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: Vec<u16>,
    my_numbers: Vec<u16>
}

fn main() {
    let cards = parse("input");
    println!("Part 1: {:?}", part1(&cards));
    println!("Part 2: {:?}", part2(&cards));
}

fn part1(cards: &[Card]) -> u32 {
    cards.iter().map(|card| {
        let mut n = 0;
        for w in &card.winning_numbers {
            if card.my_numbers.contains(w) {
                // if ugly, then ugly
                if n == 0 {
                    n = 1
                } else {
                    n *= 2
                }
            }
        }

        n
    }).sum()
}

#[test]
fn test_part1() {
    let cards = parse("test_input");
    let n = part1(&cards);

    assert_eq!(n, 13);
}

fn part2(cards: &[Card]) -> u32 {
    let mut copies = HashMap::new();

    for card in cards {
        copies.insert(card.id, 1);
    }

    for card in cards {
        let mut n = card.id;
        let c = copies[&card.id];

        for w in &card.winning_numbers {
            if card.my_numbers.contains(w) {
                n += 1;
                if let Some(t) = copies.get_mut(&n) {
                    *t += c
                }
            }
        }
    }

    copies.values().sum()
}

#[test]
fn test_part2() {
    let cards = parse("test_input");
    let n = part2(&cards);

    assert_eq!(n, 30);
}

fn to_u16_vec(string: &str) -> Vec<u16> {
    string
        .split_whitespace()
        .map(|n| n.parse::<u16>().unwrap())
        .collect()
}

fn parse(input: &'static str) -> Vec<Card> {
    let mut cards = vec![];
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    for (id, line) in reader.lines().enumerate() {
        if let Ok(l) = line {
            let (_, numbers) = l.split_once(": ").unwrap();
            let (winning, my) = numbers.split_once(" | ").unwrap();

            cards.push(
                Card {
                    id: id + 1,
                    winning_numbers: to_u16_vec(winning),
                    my_numbers: to_u16_vec(my)
                }
            );
        }
    }

    cards
}

#[test]
fn test_parse() {
    let cards = parse("test_input");
    assert_eq!(cards[0].my_numbers.len(), 8);
    assert_eq!(cards[0].winning_numbers.len(), 5);
}
