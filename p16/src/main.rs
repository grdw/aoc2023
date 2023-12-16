use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;

fn main() {
    let grid = parse("input");
    let energy_grid = walk_route(&grid);
    println!("p1: {}", energized_grid(&energy_grid));
    //println!("p2: {}", points_in_loop(&grid));
}

type Grid = Vec<Vec<char>>;
type EnergyGrid = Vec<Vec<u8>>;

#[derive(Clone, Debug)]
struct Walker {
    direction: char,
    y: isize,
    x: isize
}

fn energized_grid(energy_grid: &EnergyGrid) -> u32 {
    let mut total: u32 = 0;
    for row in energy_grid {
        for i in row {
            total += *i as u32
        }
    }
    total
}

fn walk_route(grid: &Grid) -> EnergyGrid {
    let mut energy_grid: EnergyGrid = vec![
        vec![0; grid.len()]; grid.len()
    ];

    let mut walkers = VecDeque::new();
    walkers.push_front(Walker {
        direction: 'R',
        y: 0,
        x: 0
    });

    let width = grid.len() as isize;

    while let Some(walker) = walkers.pop_front() {
        println!("{:?}", walker);
        let ny = walker.y as usize;
        let nx = walker.x as usize;
        energy_grid[ny][nx] += 1;

        // This is for debugging
        println!("");
        for e in &energy_grid {
            println!("{:?}", e);
        }
        // ^This is for debugging

        let mut direction = walker.direction;
        let mut dy = walker.y;
        let mut dx = walker.x;

        match grid[ny][nx] {
            '|'  => {
                println!("|");
                if walker.direction == 'R' || walker.direction == 'L' {
                    if walker.y < width {
                        walkers.push_back(
                            Walker {
                                direction: 'D',
                                y: walker.y + 1,
                                x: walker.x
                            }
                        );
                    }

                    if walker.y > 0 {
                        walkers.push_back(
                            Walker {
                                direction: 'U',
                                y: walker.y - 1,
                                x: walker.x
                            }
                        );
                    }
                } else {
                    match walker.direction {
                        'R' => dx += 1,
                        'L' => dx -= 1,
                        'D' => dy += 1,
                        'U' => dy -= 1,
                        _ => panic!("Unknown dircetion")
                    };

                    let limit = match walker.direction {
                        'R' => dx == (width - 1),
                        'L' => dx <= 0,
                        'D' => dy == (width - 1),
                        'U' => dy <= 0,
                        _ => panic!("Unknown dircetion")
                    };

                    if limit {
                        continue
                    }

                    walkers.push_back(Walker {
                        direction: direction,
                        y: dy,
                        x: dx
                    });
                }
            } ,
            '-'  => {
                println!("-");
                if walker.direction == 'D' || walker.direction == 'U' {
                    if walker.x > 0 {
                        walkers.push_back(
                            Walker {
                                direction: 'L',
                                y: walker.y,
                                x: walker.x - 1
                            }
                        );
                    }

                    if walker.x < width {
                        walkers.push_back(
                            Walker {
                                direction: 'R',
                                y: walker.y,
                                x: walker.x + 1
                            }
                        );
                    }
                } else {
                    match walker.direction {
                        'R' => dx += 1,
                        'L' => dx -= 1,
                        'D' => dy += 1,
                        'U' => dy -= 1,
                        _ => panic!("Unknown dircetion")
                    };

                    let limit = match walker.direction {
                        'R' => dx == (width - 1),
                        'L' => dx <= 0,
                        'D' => dy == (width - 1),
                        'U' => dy <= 0,
                        _ => panic!("Unknown dircetion")
                    };

                    if limit {
                        continue
                    }

                    walkers.push_back(Walker {
                        direction: direction,
                        y: dy,
                        x: dx
                    });
                }
            }
            '\\' => {
                println!("\\");
                direction = match direction {
                    'U' => 'L',
                    'D' => 'R',
                    'L' => 'U',
                    'R' => 'D',
                    _   => panic!("boom")
                };
                match direction {
                    'R' => dx += 1,
                    'L' => dx -= 1,
                    'D' => dy += 1,
                    'U' => dy -= 1,
                    _ => panic!("Unknown dircetion")
                };

                let limit = match direction {
                    'R' => dx == (width - 1),
                    'L' => dx <= 0,
                    'D' => dy == (width - 1),
                    'U' => dy <= 0,
                    _ => panic!("Unknown dircetion")
                };

                if limit {
                    continue
                }
                walkers.push_back(Walker {
                    direction: direction,
                    y: dy,
                    x: dx
                });
            },
            '/'  => {
                println!("/");
                direction = match direction {
                    'U' => 'R',
                    'D' => 'L',
                    'L' => 'D',
                    'R' => 'U',
                    _   => panic!("boom")
                };
                match direction {
                    'R' => dx += 1,
                    'L' => dx -= 1,
                    'D' => dy += 1,
                    'U' => dy -= 1,
                    _ => panic!("Unknown dircetion")
                };

                let limit = match direction {
                    'R' => dx == (width - 1),
                    'L' => dx <= 0,
                    'D' => dy == (width - 1),
                    'U' => dy <= 0,
                    _ => panic!("Unknown dircetion")
                };

                if limit {
                    continue
                }
                walkers.push_back(Walker {
                    direction: direction,
                    y: dy,
                    x: dx
                });
            },
            '.'  => {
                match walker.direction {
                    'R' => dx += 1,
                    'L' => dx -= 1,
                    'D' => dy += 1,
                    'U' => dy -= 1,
                    _ => panic!("Unknown dircetion")
                };

                let limit = match walker.direction {
                    'R' => dx == (width - 1),
                    'L' => dx <= 0,
                    'D' => dy == (width - 1),
                    'U' => dy <= 0,
                    _ => panic!("Unknown dircetion")
                };

                if limit {
                    continue
                }

                walkers.push_back(Walker {
                    direction: direction,
                    y: dy,
                    x: dx
                });
            },
            _    => panic!("Unknown tile")
        }
    }

    energy_grid
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
fn test_walk_route() {
    let grid = parse("test_input");
    let energy_grid = walk_route(&grid);
    assert_eq!(energized_grid(&energy_grid), 46);
}
