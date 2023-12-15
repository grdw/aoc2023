use std::fs;

fn main() {
    let list = parse("input");
    println!("p1: {}", sum(&list));
}

const MAX_FACTOR: u16 = 17;
const REMAINDER: u16 = 256;

fn sum(l: &Vec<String>) -> u32 {
    l.iter().map(|n| hash_string(&n) as u32 ).sum()
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
    println!("{:?}", list);
    assert_eq!(sum(&list), 1320);
}
