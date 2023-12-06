use std::fs;

struct Race {
    time: u16,
    distance: u16
}

fn main() {
    let races = parse("input");
    println!("The answer to part1 is: {}", part1(&races));
    println!("The answer to part2 is: {}", part2(&races))
}

fn part1(races: &[Race]) -> u16 {
    0
}

#[test]
fn test_part1() {
    let races = parse("test_input");
    assert_eq!(part1(&races), 142)
}

fn part2(races: &[Race]) -> u16 {
    0
}

#[test]
fn test_part2() {
    let races = parse("test_input");
    assert_eq!(part2(&races), 444)
}

fn parse(input: &'static str) -> Vec<Race> {
    let mut races: Vec<Race> = vec![];
    let d = fs::read_to_string(input).unwrap();
    for l in d.split_terminator("\n") {
        for (x, d) in l.split_whitespace().enumerate() {
            if x < 1 { continue }

            let index = x - 1;
            let v = d.parse::<u16>().unwrap();

            match races.get_mut(index) {
                Some(race) => { race.distance = v },
                None => races.insert(
                    index,
                    Race { time: v, distance: 0 }
                )
            }
        }
    }
    races
}

#[test]
fn test_parse() {
    let races = parse("test_input");
    assert_eq!(races.len(), 3);
    assert_eq!(races[0].time, 7);
    assert_eq!(races[0].distance, 9);
}
