use num_integer::lcm;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

type Route = HashMap<String, (String, String)>;

fn main() {
    let (lr, route) = parse("input");
    println!("p1: {:?}", navigate(&route, &lr, "AAA", "ZZZ").unwrap());
    println!("p2: {:?}", part2(&route, &lr));
}

fn navigate(route: &Route, lr: &String, from: &str, to: &str) -> Option<u64> {
    let mut i = 0;
    let mut c = lr.chars().cycle();
    let mut start = from;

    while start != to {
        let dir = c.next().unwrap();
        let (l, r) = route.get(start).unwrap();
        // TODO:
        // This is a bit of an ugly hack ... but it works ;)
        if i as usize > (lr.len().pow(2)) + 1 {
            return None
        }

        if dir == 'L' {
            start = l;
        } else if dir == 'R' {
            start = r;
        }
        i += 1;
    }
    Some(i)
}

fn part2(route: &Route, lr: &String) -> u64 {
    let starts: Vec<&String> = route
        .keys()
        .filter(|n| n.ends_with("A"))
        .collect();

    let ends: Vec<&String> = route
        .keys()
        .filter(|n| n.ends_with("Z"))
        .collect();

    let mut t = vec![];
    for s in &starts {
        for e in &ends {
            if let Some(n) = navigate(route, lr, s, e) {
                t.push(n)
            }
        }
    }

    t.iter().fold(1, |acc, &x| lcm(acc, x))
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
fn test_navigate() {
    let (lr, route) = parse("test_input");
    assert_eq!(navigate(&route, &lr, "AAA", "ZZZ"), Some(2));
    let (lr, route) = parse("test_input2");
    assert_eq!(navigate(&route, &lr, "AAA", "ZZZ"), Some(6));
}

#[test]
fn test_part2() {
    let (lr, route) = parse("test_input3");
    assert_eq!(part2(&route, &lr), 6);
}
