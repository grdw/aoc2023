use std::fs;

struct Race {
    time: u64,
    distance: u64
}

impl Race {
    fn resolve(&self) -> u64 {
        (0..self.time)
            .filter(|speed| (self.time - speed) * speed > self.distance)
            .count() as u64
    }
}

fn main() {
    let (races, race) = parse("input");
    println!("The answer to part1 is: {}", part1(&races));
    println!("The answer to part2 is: {}", part2(&race))
}

fn part1(races: &[Race]) -> u64 {
    races.iter().map(|r| r.resolve()).product()
}

#[test]
fn test_part1() {
    let (races, _) = parse("test_input");
    assert_eq!(part1(&races), 288)
}

fn part2(race: &Race) -> u64 {
    race.resolve()
}

#[test]
fn test_part2() {
    let (_, race) = parse("test_input");
    assert_eq!(part2(&race), 71503)
}

fn parse(input: &'static str) -> (Vec<Race>, Race) {
    let mut race = Race { time: 0, distance: 0 };
    let mut races: Vec<Race> = vec![];
    let d = fs::read_to_string(input).unwrap();

    for l in d.split_terminator('\n') {
        let (name, ns) = l.split_once(':').unwrap();
        let n = ns.replace(' ', "");
        let v = n.parse::<u64>().unwrap();

        match name {
            "Time" => race.time = v,
            "Distance" => race.distance = v,
            _ => panic!("Invalid name")
        }

        for (x, d) in l.split_whitespace().enumerate() {
            if x < 1 { continue }

            let index = x - 1;
            let v = d.parse::<u64>().unwrap();

            match races.get_mut(index) {
                Some(race) => { race.distance = v },
                None => races.insert(
                    index,
                    Race { time: v, distance: 0 }
                )
            }
        }
    }
    (races, race)
}

#[test]
fn test_parse() {
    let (races, _) = parse("test_input");
    assert_eq!(races.len(), 3);
    assert_eq!(races[0].time, 7);
    assert_eq!(races[0].distance, 9);
}
