use std::fs;
use std::collections::HashMap;

const RADIX: u32 = 10;

fn main() {
    println!("The answer to part1 is: {}", part1("input"));
    println!("The answer to part2 is: {}", part2("input"))
}

fn part1(input: &'static str) -> u16 {
    let s = fs::read_to_string(input).unwrap();
    return s.split_terminator("\n").map(|line| {
        let mut l = ' ';
        let mut r = ' ';
        let lc = line.as_bytes();

        for i in 0..line.len() {
            let c = lc[i] as char;
            let d = lc[lc.len() - 1 - i] as char;

            if c.is_digit(RADIX) && l == ' ' {
                l = c;
            }

            if d.is_digit(RADIX) && r == ' ' {
                r = d;
            }
        }

        let t = format!("{}{}", l, r);
        t.parse::<u16>().unwrap()
    }).sum::<u16>()
}

#[test]
fn test_part1() {
    assert_eq!(part1("test_input"), 142)
}

fn part2(input: &'static str) -> u16 {
    let s = fs::read_to_string(input).unwrap();
    let matches = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    return s.split_terminator("\n").map(|line| {
        let mut li = line.len();
        let mut ri = 0;
        let mut l = "";
        let mut r = "";

        for k in matches.keys() {
            for (fi, fv) in line.match_indices(k) {
                if fi < li {
                    li = fi;
                    l = fv;
                }

                if fi >= ri {
                    ri = fi;
                    r = fv;
                }
            }
        }

        let t = format!("{}{}", matches[l], matches[r]);
        t.parse::<u16>().unwrap()
    }).sum::<u16>()
}

#[test]
fn test_part2() {
    assert_eq!(part2("test_input2"), 444)
}
