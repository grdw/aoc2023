fn main() {
    println!("Hello, world!");
}

const MAX_FACTOR: u16 = 17;
const REMAINDER: u16 = 256;

fn hash(current_value: &mut u16, c: char) -> u16 {
    let as_ascii = (c as u8) as u16;
    *current_value += as_ascii;
    *current_value *= MAX_FACTOR;
    *current_value % REMAINDER
}

fn hash_string(s: &str) -> u16 {
   s.chars().fold(0, |mut acc, x| hash(&mut acc, x))
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
