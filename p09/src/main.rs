use std::fs::File;
use std::io::{BufReader, BufRead};

type History = Vec<Vec<i32>>;

fn main() {
    let history = parse("input");
    let (lefts, rights) = make_ends(&history);
    println!("p1: {:?}", part1(&rights));
    println!("p2: {:?}", part2(&lefts));
}

fn part1(history: &History) -> i32 {
    history
        .iter()
        .map(|h| h.iter().sum::<i32>())
        .sum::<i32>()
}

fn part2(history: &History) -> i32 {
    history
        .iter()
        .map(|h| h.iter().rev().fold(0, |acc, x| x - acc))
        .sum::<i32>()
}

fn make_ends(history: &History) -> (History, History) {
    let mut lefts = vec![];
    let mut rights = vec![];

    for line in history {
        let mut l = vec![];
        let mut r = vec![];
        let mut differences = line.clone();

        loop {
            l.push(differences[0]);
            r.push(differences[differences.len() - 1]);

            differences = (0..(differences.len() - 1))
                .map(|i| differences[i + 1] - differences[i])
                .collect();

            if differences.iter().all(|&n| n == 0) {
                break
            }
        }
        lefts.push(l);
        rights.push(r);
    }

    (lefts, rights)
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
    let (_, rights) = make_ends(&history);
    assert_eq!(part1(&rights), 114);
}

#[test]
fn test_part2() {
    let history = parse("test_input");
    let (lefts, _) = make_ends(&history);
    assert_eq!(part2(&lefts), 2);
}
