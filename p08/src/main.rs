use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

type Route = HashMap<String, (String, String)>;

fn main() {
    let (lr, route) = parse("input");
    println!("p1: {:?}", part1(&route, &lr));
    println!("p2: {:?}", part2(&route, &lr));
}

fn part1(route: &Route, lr: &String) -> u64 {
    let mut i = 0;
    let mut c = lr.chars().cycle();
    let mut start = "AAA";

    while start != "ZZZ" {
        let dir = c.next().unwrap();
        let (l, r) = route.get(start).unwrap();
        if dir == 'L' {
            start =  l;
        } else if dir == 'R' {
            start = r;
        }
        i += 1;
    }
    i
}

fn part2(route: &Route, lr: &String) -> u64 {
    let mut i = 0;
    let mut c = lr.chars().cycle();

    let mut starts: Vec<&String> = route
        .keys()
        .filter(|n| n.ends_with("A"))
        .collect();

    while !starts.iter().all(|n| n.ends_with("Z")) {
        let dir = c.next().unwrap();
        for s in starts.iter_mut() {
            let key = s.clone();
            let (l, r) = route.get(&key).unwrap();
            if dir == 'L' {
                *s = l;
            } else if dir == 'R' {
                *s = r;
            }
        }
        i += 1;
    }
    i
}

fn parse(input: &'static str) -> (String, Route) {
    let file = File::open(input).unwrap();
    let mut lines = BufReader::new(file).lines();

    let l = lines.next().unwrap().unwrap();
    let mut map = Route::new();

    for line in lines {
        let l = line.unwrap();
        if l == "" {
            continue
        }

        let (main, left_right) = l.split_once(" = ").unwrap();
        let (left, right) = left_right.split_once(", ").unwrap();
        map.insert(
            main.to_string(),
            (left[1..].to_string(), right[0..3].to_string())
        );
    }

    (l, map)
}

#[test]
fn test_parse() {
    let (lr, route) = parse("test_input");
    assert_eq!(lr, "RL");
    assert_eq!(route["AAA"], ("BBB".to_string(), "CCC".to_string()));
}

#[test]
fn test_part1() {
    let (lr, route) = parse("test_input");
    assert_eq!(part1(&route, &lr), 2);
    let (lr, route) = parse("test_input2");
    assert_eq!(part1(&route, &lr), 6);
}

#[test]
fn test_part2() {
    let (lr, route) = parse("test_input3");
    assert_eq!(part2(&route, &lr), 6);
}
