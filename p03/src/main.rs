use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::Range;

struct Grid {
    numbers: Vec<Number>,
    specials: Vec<Special>
}

struct Number {
    n: u32,
    y: usize,
    x: Range<usize>
}

struct Special {
    c: char,
    x: usize,
    y: usize
}

impl Special {
    fn is_match(&self, n: &Number) -> bool {
        let by = self.y == n.y || self.y + 1 == n.y || self.y - 1 == n.y;
        let bx = n.x.contains(&self.x) ||
                 n.x.contains(&(self.x + 1)) ||
                 n.x.contains(&(self.x - 1));

        bx && by
    }
}

fn main() {
    let grid = parse("input");
    println!("Part 1: {:?}", part1(&grid));
    println!("Part 2: {:?}", part2(&grid));
}

fn part1(grid: &Grid) -> u32 {
    grid.numbers
        .iter()
        .filter(|n| grid.specials.iter().any(|s| s.is_match(n)))
        .map(|n| n.n)
        .sum()
}

#[test]
fn test_part1() {
    let grid = parse("test_input");
    let n = part1(&grid);

    assert_eq!(n, 4361);
}

fn part2(grid: &Grid) -> u32 {
    let mut t = 0;

    for s in &grid.specials {
        let mut numbers = vec![];
        for n in &grid.numbers {
            if s.is_match(n) && s.c == '*' {
                numbers.push(n.n);
            }
        };

        if numbers.len() == 2 {
            t += numbers[0] * numbers[1]
        }
    }

    t
}

#[test]
fn test_part2() {
    let grid = parse("test_input");
    let n = part2(&grid);

    assert_eq!(n, 467835);
}

fn parse(input: &'static str) -> Grid {
    let mut grid = Grid { numbers: vec![], specials: vec![] };
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let rang = '0'..='9';

    for (y, line) in reader.lines().enumerate() {
        let mut result = String::new();

        for (x, c) in line.expect("").chars().enumerate() {
            if rang.contains(&c) {
                result.push(c);
            } else if !result.is_empty() {
                let n = result.parse::<u32>().unwrap();
                let x = (x-result.len())..x;

                grid.numbers.push(Number { y, x, n });
                result.clear();
            }

            if c != '.' && !rang.contains(&c) {
                grid.specials.push(Special { y, x, c });
            }
        }
    }

    grid
}

#[test]
fn test_parse() {
    let grid = parse("test_input");
    assert_eq!(grid.numbers.len(), 10);
    assert_eq!(grid.specials.len(), 6);
}
