use std::cmp;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let grid = parse("input");
    let l_path = find_loop(&grid);
    println!("p1: {}", furthest_point(&l_path));
    println!("p2: {}", points_in_loop(&l_path));
}

type Grid = Vec<Vec<char>>;
type Loop = Vec<(usize, usize)>;

fn furthest_point(l: &Loop) -> usize {
    l.len() / 2
}

fn find_loop(grid: &Grid) -> Loop {
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

            // Out of bounds
            if dx > grid[0].len() || dy > grid.len() {
                continue
            }

            // Same point as the last one
            // There's probably something clever to be done here..
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

    l_path
}

fn points_in_loop(l: &Loop) -> u32 {
    let (mut minx, mut miny) = (usize::MAX, usize::MAX);
    let (mut maxx, mut maxy) = (0, 0);
    for (y, x) in l {
        minx = cmp::min(minx, *x);
        miny = cmp::min(miny, *y);
        maxx = cmp::max(maxx, *x);
        maxy = cmp::max(maxy, *y);
    }

    let mut count_grid: Vec<Vec<usize>> = vec![];
    for _ in 0..=maxy {
        let mut v = vec![];
        for _ in 0..=maxx {
            v.push(0);
        }
        count_grid.push(v);
    }

    let mut count = 0;
    let diffs: Vec<(isize, isize)> = vec![(-1, 0),(0, -1),(0, 1),(1, 0)];
    println!("=====");

    for y in miny..=maxy {
        for x in minx..=maxx {
            if l.iter().any(|(ly, lx)| ly == &y && lx == &x) {
                continue
            }

            println!("{} {}", y, x);
            for (dy, dx) in &diffs {
                let dx = ((x as isize) + dx) as usize;
                let dy = ((y as isize) + dy) as usize;

                // Out of bounds
                if dx > maxx || dy > maxy {
                    continue
                }

                count_grid[y][x] += l.iter()
                    .filter(|(ly, lx)| ly == &dy && lx == &dx)
                    .count();
            }
        }
    }

    println!("=====");
    for g in &count_grid {
        println!("{:?}", g);
    }

    for g in &count_grid {
        for y in g {
            if y > &2 {
                count += 1;
            }
        }
    }

    count
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
    let l_path = find_loop(&grid);
    assert_eq!(furthest_point(&l_path), 4);

    let grid = parse("test_input");
    let l_path = find_loop(&grid);
    assert_eq!(furthest_point(&l_path), 8);
}

#[test]
fn test_points_in_loop() {
    let grid = parse("test_input3");
    let l_path = find_loop(&grid);
    assert_eq!(points_in_loop(&l_path), 4);

    let grid = parse("test_input5");
    let l_path = find_loop(&grid);
    assert_eq!(points_in_loop(&l_path), 4);
}

#[test]
fn breaky() {
    let grid = parse("test_input4");
    let l_path = find_loop(&grid);
    assert_eq!(points_in_loop(&l_path), 8);
}
