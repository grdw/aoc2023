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
        let mut diffs = line.clone();

        loop {
            let li = diffs.len() - 1;
            l.push(diffs[0]);
            r.push(diffs[li]);

            diffs = (0..li)
                .map(|i| diffs[i + 1] - diffs[i])
                .collect();

            if diffs.iter().all(|&n| n == 0) {
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
        line
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect()
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
