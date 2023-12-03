use regex::Regex;
use std::fs;
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
    x: usize,
    y: usize
}

fn main() {
    let grid = parse("input");
    println!("Part 1: {:?}", part1(&grid));
}

fn part1(grid: &Grid) -> u32 {
    grid.numbers
        .iter()
        .filter(|n| {
            grid.specials.iter().any(|s| {
                let by = s.y == n.y || s.y + 1 == n.y || s.y - 1 == n.y;
                let bx = n.x.contains(&s.x) ||
                         n.x.contains(&(s.x + 1)) ||
                         n.x.contains(&(s.x - 1));

             bx && by
            })
        })
        .map(|n| n.n)
        .sum()
}

#[test]
fn test_part1() {
    let grid = parse("test_input");
    let n = part1(&grid);

    assert_eq!(n, 4361);
}

fn parse(input: &'static str) -> Grid {
    let mut grid = Grid { numbers: vec![], specials: vec![] };
    let f = fs::read_to_string(input).unwrap();
    let re = Regex::new(r"[0-9]+|[^0-9\.]").unwrap();

    for (y, s) in f.split_terminator("\n").enumerate() {
        for m in re.find_iter(s) {
            match m.as_str().parse::<u32>() {
                Ok(n) => {
                    grid.numbers.push(Number {
                        n: n,
                        y: y,
                        x: (m.start()..m.end())
                    });
                },
                Err(_) => {
                    grid.specials.push(Special {
                        y: y,
                        x: m.start()
                    });
                }
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
