use std::fs;
use std::collections::HashMap;

fn main() {
    let list = parse("input");
    println!("p1: {}", sum(&list));
    println!("p2: {}", focussing_power(&list));
}

const MAX_FACTOR: u16 = 17;
const REMAINDER: u16 = 256;

fn sum(l: &Vec<String>) -> u32 {
    l.iter().map(|n| hash_string(&n) as u32).sum()
}

fn focussing_power(l: &Vec<String>) -> u32 {
    let mut map: HashMap<u16, Vec<(&str, u16)>> = HashMap::new();

    for n in l {
        let (action, label, focal_length) = match n.split_once("=") {
            Some(label) => {
                ("add", label.0, label.1.parse::<u16>().unwrap())
            }
            None => {
                let (label, _) = n.split_once("-").unwrap();
                ("remove", label, 0)
            }
        };

        let bnr = hash_string(label);
        let entry = map.entry(bnr);

        match action {
            "add" => {
                entry
                    .and_modify(|m| {
                        match m.iter().position(|(x, _)| *x == label) {
                            Some(x) => {
                                m.remove(x);
                                m.insert(x, (&label, focal_length));
                            },
                            None => m.push((&label, focal_length))
                        }
                    })
                    .or_insert(vec![(label, focal_length)]);
            },
            "remove" => {
                entry.and_modify(|m| {
                    if let Some(x) = m.iter().position(|(x, _)| *x == label) {
                        m.remove(x);
                    }
                });
            },
            _ => panic!("wrong action buddy")
        }
    }

    map.iter().map(|(k, v)| {
        v.iter().enumerate().map(|(i, m)| {
            ((k + 1) * (i + 1) as u16 * m.1) as u32
        }).sum::<u32>()
    }).sum::<u32>()
}

fn hash(current_value: &mut u16, c: char) -> u16 {
    let as_ascii = (c as u8) as u16;
    *current_value += as_ascii;
    *current_value *= MAX_FACTOR;
    *current_value % REMAINDER
}

fn hash_string(s: &str) -> u16 {
   s.chars().fold(0, |mut acc, x| hash(&mut acc, x))
}

fn parse(input: &'static str) -> Vec<String> {
    let list = fs::read_to_string(input).unwrap();
    list
        .split_terminator(",")
        .map(|i| i.trim().to_string())
        .collect::<Vec<String>>()
}

#[test]
fn test_hash() {
    assert_eq!(hash(&mut 0, 'a'), 113);
    assert_eq!(hash(&mut 0, 'H'), 200);
}

#[test]
fn test_hash_string() {
    assert_eq!(hash_string("HASH"), 52);
}

#[test]
fn test_sum() {
    let list = parse("test_input");
    assert_eq!(sum(&list), 1320);
}

#[test]
fn test_focussing_power() {
    let list = parse("test_input");
    assert_eq!(focussing_power(&list), 145);
}
