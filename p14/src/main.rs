use std::fs::File;
use std::io::{BufReader, BufRead};

type Grid = Vec<Vec<char>>;

fn main() {
    let mut grid = parse("input");
    println!("p1: {}", total_load(&mut grid));
    //println!("p2: {}", points_in_loop(&l_path));
}

fn total_load(grid: &mut Grid) -> usize {
    let mut rocks: Vec<(usize, usize)> = vec![];
    for (y, yrow) in grid.iter().enumerate() {
        for (x, o) in yrow.iter().enumerate() {
            if o == &'O' {
                rocks.push((y, x));
            }
        }
    }

    for (y, x) in rocks {
        let mut n = y;
        loop {
            if n == 0 { break; }
            if grid[n - 1][x] == 'O' || grid[n - 1][x] == '#' {
                break;
            }

            grid[n][x] = '.';
            n -= 1;
            grid[n][x] = 'O';
        }
    }

    grid.iter().enumerate().map(|(n, g)| {
        g.iter().filter(|&&n| n == 'O').count() * (grid.len() - n)
    }).sum::<usize>()
}

fn parse(input: &'static str) -> Grid {
    let file = File::open(input).unwrap();
    let lines = BufReader::new(file).lines();
    let mut grid: Grid = vec![];

    for line in lines {
        let l = line.unwrap();

        grid.push(l.chars().collect());
    }

    grid
}

#[test]
fn test_parse() {
    let grid = parse("test_input");
    assert_eq!(grid.len(), 10);
    assert_eq!(grid[0].len(), 10);
}

#[test]
fn test_total_load() {
    let mut grid = parse("test_input");
    assert_eq!(total_load(&mut grid), 136);
}
