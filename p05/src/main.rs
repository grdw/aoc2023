use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug)]
struct Translation {
    source_range: Range<i64>,
    t: i64
}

#[derive(Debug)]
struct SourceMap {
    to: String,
    translations: Vec<Translation>
}

impl SourceMap {
    fn new(to: &str) -> SourceMap {
        SourceMap { to: String::from(to), translations: vec![] }
    }

    fn get(&self, n: i64) -> i64 {
        for trans in &self.translations {
            if trans.source_range.contains(&n) {
                return n + trans.t
            }
        }
        n
    }
}

#[test]
fn test_source_map() {
    let sm = SourceMap {
        to: String::from("soil"),
        translations: vec![
            Translation { source_range: 98..100, t: -48 },
            Translation { source_range: 50..98, t: 2 },
        ]
    };
    assert_eq!(sm.get(0), 0);
    assert_eq!(sm.get(1), 1);
    assert_eq!(sm.get(48), 48);
    assert_eq!(sm.get(50), 52);
    assert_eq!(sm.get(51), 53);
    assert_eq!(sm.get(96), 98);
    assert_eq!(sm.get(97), 99);
    assert_eq!(sm.get(98), 50);
    assert_eq!(sm.get(99), 51);
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: HashMap<String, SourceMap>
}

fn main() {
    let almanac = parse("input");
    println!("Part 1: {:?}", part1(&almanac));
    println!("Part 2: {:?}", part2(&almanac));
}

fn part1(almanac: &Almanac) -> i64 {
    let mut min = i64::MAX;

    for seed in &almanac.seeds {
        let mut key = "seed";
        let mut value = *seed;

        while key != "location" {
            let map = &almanac.maps[key];

            value = map.get(value);
            key = map.to.as_str();
        }

        if value < min {
            min = value
        }
    }

    min
}

#[test]
fn test_part1() {
    let almanac = parse("test_input");
    let n = part1(&almanac);

    assert_eq!(n, 35);
}

fn part2(almanac: &Almanac) -> i64 {
    return 0
}

#[test]
fn test_part2() {
    let almanac = parse("test_input");
    let n = part2(&almanac);

    assert_eq!(n, 1);
}

fn to_i64_vec(s: &str) -> Vec<i64> {
    s.split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect()
}

fn parse(input: &'static str) -> Almanac {
    let mut seeds = vec![];
    let mut maps = HashMap::new();
    let mut almanac = Almanac { seeds, maps };
    let mut key = String::new();
    let file = File::open(input).unwrap();
    let mut lines = BufReader::new(file).lines();

    if let Some(l) = lines.next() {
        let line = l.unwrap();
        let (_, s) = line.split_once(" ").unwrap();

        almanac.seeds = to_i64_vec(s);
    }

    for line in lines {
        let l = line.unwrap();

        if l == "" {
            continue
        }

        if l.chars().next().unwrap().is_numeric() {
            let p = to_i64_vec(&l);
            if let Some(map) = almanac.maps.get_mut(&key) {
                map.translations.push(
                    Translation {
                        source_range: p[1]..p[1]+p[2],
                        t: p[0]-p[1]
                    }
                )
            }
        } else {
            let (n, _) = l.split_once(" ").unwrap();
            let (from, to) = n.split_once("-to-").unwrap();

            key = String::from(from);
            almanac.maps.insert(key.clone(), SourceMap::new(to));
        }
    }

    almanac
}

#[test]
fn test_parse() {
    let almanac = parse("test_input");
    assert_eq!(almanac.seeds.len(), 4);
    assert_eq!(almanac.maps.keys().len(), 7);
}
