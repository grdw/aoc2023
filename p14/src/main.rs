use std::fs::File;
use std::io::{BufReader, BufRead};

type Grid = Vec<Vec<char>>;

fn main() {
    let mut grid = parse("input");
    tumble(&mut grid);
    println!("p1: {}", total_load(&grid));

    let mut grid = parse("input");
    tumble_directional(&mut grid);
    println!("p2: {}", total_load(&grid));
}

fn tumble(grid: &mut Grid) {
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
}

fn tumble_directional(grid: &mut Grid) {
    let mut rocks: Vec<(usize, usize)> = vec![];
    for (y, yrow) in grid.iter().enumerate() {
        for (x, o) in yrow.iter().enumerate() {
            if o == &'O' {
                rocks.push((y, x));
            }
        }
    }

    let directions = ['N', 'W', 'S', 'E'];
    let height = grid.len();
    let width = grid[0].len();

    for _ in 0..1 {
        for d in directions {
            for (y, x) in &rocks {
                let mut nx = *x;
                let mut ny = *y;

                loop {
                    let premise = match d {
                        'N' => ny >= height - 1,
                        'W' => nx == 0,
                        'E' => nx >= width - 1,
                        'S' => ny == 0,
                        _ => false
                    };

                    if premise {
                        break;
                    }

                    //if grid[ny][nx] == 'O' || grid[ny][nx] == '#' {
                    //    break;
                    //}

                    grid[ny][nx] = '.';
                    match d {
                        'N' => ny += 1,
                        'W' => nx -= 1,
                        'S' => ny -= 1,
                        'E' => nx += 1,
                        _ => panic!("invalid")
                    }
                    grid[ny][nx] = 'O';
                }
            }

            println!("============= {}", d);
            for g in &*grid {
                println!("{}", g.iter().collect::<String>());
            }
        }

    }
}

fn total_load(grid: &Grid) -> usize {
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
    tumble(&mut grid);
    assert_eq!(total_load(&mut grid), 136);
}

#[test]
fn test_total_load_tumble_dir() {
    let mut grid = parse("test_input");
    tumble_directional(&mut grid);
    assert_eq!(total_load(&mut grid), 64);
}
