use regex::Regex;
use std::fs;

fn main() {
    let games = parse("input");
    println!("Part1: {:?}", part1(&games, 12, 13, 14));
    println!("Part2: {:?}", part2(&games));
}

#[derive(Debug)]
struct ElfHand {
    blue: u32,
    red: u32,
    green: u32
}

#[derive(Debug)]
struct Game {
    id: u16,
    hands: Vec<ElfHand>
}

fn parse(file: &'static str) -> Vec<Game> {
    let contents = fs::read_to_string(file).unwrap();
    let re = Regex::new(r"Game (-?\d+): (.+)").unwrap();

    contents.split_terminator("\n").map(|line| {
        let caps = re.captures(line).unwrap();
        let hands: Vec<ElfHand> = caps[2]
            .split_terminator("; ")
            .map(|hand|{
                let mut elf_hand = ElfHand {
                    blue: 0,
                    red: 0,
                    green: 0
                };

                for s in hand.split_terminator(", ") {
                    let cubes: Vec<&str> = s.split(" ").collect();
                    let n = cubes[0].parse::<u32>().unwrap();
                    let m = cubes[1];

                    match m {
                        "red"   => elf_hand.red = n,
                        "blue"  => elf_hand.blue = n,
                        "green" => elf_hand.green = n,
                        _       => panic!("invalid color")
                    }
                }
                elf_hand
            }).collect();

        Game {
            id: caps[1].parse::<u16>().unwrap(),
            hands: hands
        }
    }).collect()
}


#[test]
fn test_parse() {
    assert_eq!(parse("test_input").len(), 5)
}

fn part1(games: &Vec<Game>, red: u32, green: u32, blue: u32) -> u16 {
    return games
        .iter()
        .filter(|g| {
            g.hands.iter().all(|h|{
                h.blue <= blue && h.green <= green && h.red <= red
            })
        })
        .map(|g| g.id)
        .sum()
}

#[test]
fn test_part1() {
    let cubes = parse("test_input");
    assert_eq!(part1(&cubes, 12, 13, 14), 8)
}

fn part2(games: &Vec<Game>) -> u32 {
    let mut t = 0;
    for g in games {
        let mut max_hand = ElfHand { blue: 0, green: 0, red: 0 };

        for h in &g.hands {
            if h.blue > max_hand.blue {
                max_hand.blue = h.blue
            }

            if h.green > max_hand.green {
                max_hand.green = h.green
            }

            if h.red > max_hand.red {
                max_hand.red = h.red
            }
        }

        t += max_hand.red * max_hand.green * max_hand.blue
    }
    return t
}

#[test]
fn test_part2() {
    let cubes = parse("test_input");
    assert_eq!(part2(&cubes), 2286)
}
