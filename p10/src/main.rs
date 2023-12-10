use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let grid = parse("input");
    println!("p1: {}", furthest_point(&grid));
    println!("p1: {}", part2(&grid));
}

type Grid = Vec<Vec<char>>;

fn furthest_point(grid: &Grid) -> usize {
    let (mut startx, mut starty) = (0, 0);

    let parts = HashMap::from([
        ('|', vec![(1, 0), (-1, 0)]),
        ('-', vec![(0, -1), (0, 1)]),
        ('L', vec![(-1, 0), (0, 1)]),
        ('J', vec![(-1, 0), (0, -1)]),
        ('7', vec![(1, 0), (0, -1)]),
        ('F', vec![(1, 0), (0, 1)]),
        ('S', vec![(-1, 0),(0, -1),(0, 1),(1, 0)]),
        ('.', vec![])
    ]);

    for (y, row) in grid.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            if p == &'S' {
                startx = x;
                starty = y;
                break;
            }
        }
    }

    let mut l_path = vec![];
    l_path.push((starty, startx));

    let mut index = 0;
    let mut current_part = 'S';

    loop {
        let (sy, sx) = l_path[index];

        if sy == starty && sx == startx && index > 0 {
            break;
        }

        'outer: for (y, x) in &parts[&current_part] {
            let dx = ((sx as isize) + x) as usize;
            let dy = ((sy as isize) + y) as usize;
            if dx > grid.len() || dy > grid.len() {
                continue
            }

            if index > 0 {
                let (prevy, prevx) = l_path[index - 1];
                if prevy == dy && prevx == dx {
                    continue;
                }
            }

            let k = grid[dy][dx];
            for (yy, xx) in &parts[&k] {
                let ddx = dx as isize + xx;
                let ddy = dy as isize + yy;

                if ddx == sx as isize && ddy == sy as isize {
                    l_path.push((dy, dx));
                    current_part = k;
                    index += 1;
                    break 'outer;
                }
            }
        }
    }

    l_path.len() / 2
}

fn part2(grid: &Grid) -> u32 {
    0
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
    let grid = parse("test_input2");
    assert_eq!(grid.len(), 5);
    assert_eq!(grid[0].len(), 5);
}

#[test]
fn test_furthest_point() {
    let grid = parse("test_input2");
    assert_eq!(furthest_point(&grid), 4);
    let grid = parse("test_input");
    assert_eq!(furthest_point(&grid), 8);
}
