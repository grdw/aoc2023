use std::fs::File;
use std::io::{BufReader, BufRead};

type History = Vec<Vec<i32>>;

fn main() {
    let history = parse("input");
    println!("p1: {:?}", part1(&history));
    println!("p2: {:?}", part2(&history));
}

fn part1(history: &History) -> i32 {
    history
        .iter()
        .map(|line| {
            let mut ends = vec![line[line.len() - 1]];
            let mut differences = line.clone();

            while !differences.iter().all(|&n| n == 0) {
                let mut a = vec![];
                for i in 0..(differences.len() - 1) {
                    a.push(differences[i + 1] - differences[i]);
                }
                differences = a;
                ends.push(differences[differences.len() - 1]);
            }

            ends.iter().sum::<i32>()
        }).sum::<i32>()
}

fn part2(history: &History) -> i32 {
    0
}

fn parse(input: &'static str) -> History {
    let file = File::open(input).unwrap();
    let lines = BufReader::new(file).lines();

    lines.map(|line| {
        let l = line.unwrap();
        l.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect()
    }).collect()
}

#[test]
fn test_parse() {
    let history = parse("test_input");
    assert_eq!(history.len(), 3);
}

#[test]
fn test_part1() {
    let history = parse("test_input");
    assert_eq!(part1(&history), 114);
}

#[test]
fn test_part2() {
    let history = parse("test_input");
    assert_eq!(part2(&history), 114);
}
