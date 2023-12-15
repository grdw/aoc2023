use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct SpringCondition {
    template: String,
    group: Vec<u8>
}

fn main() {
    let spring_conditions = parse("input");
    println!("p1: {}", combinations(&spring_conditions));
    //println!("p2: {}", points_in_loop(&l_path));
}

fn combination(s: &SpringCondition) -> u32 {
    let v: Vec<_> = s.template.match_indices('?').collect();
    let mut t = 0;
    let mut new_s: Vec<char> = s.template.chars().collect();

    let x = 2_u32.pow(v.len() as u32);

    for n in 0..x {
        let f = format!("{:0>width$b}", n, width = v.len());
        let mut n = 0;
        let mut pattern = vec![];

        for i in 0..v.len() {
            let p = f.chars().nth(i).unwrap_or('0');
            let q = match p {
                '1' => '#',
                '0' => '.',
                _ => panic!("boom")
            };

            let j = v[i].0;
            new_s[j] = q
        }

        for s in 0..=new_s.len() {
            let c = new_s.get(s).unwrap_or(&'.');
            if c == &'#' {
                n += 1
            } else {
                if n > 0 {
                    pattern.push(n);
                }
                n = 0;
            }
        }

        if pattern == s.group {
            t += 1
        }
    }

    t
}

fn combinations(a: &Vec<SpringCondition>) -> u32 {
    a.iter().map(|n| combination(n)).sum()
}

fn parse(input: &'static str) -> Vec<SpringCondition> {
    let file = File::open(input).unwrap();
    let lines = BufReader::new(file).lines();
    let mut spring_conditions = vec![];

    for line in lines {
        let l = line.unwrap();
        let (template, group) = l.split_once(" ").unwrap();

        spring_conditions.push(SpringCondition{
            template: String::from(template),
            group: group.split(",").map(|i| i.parse::<u8>().unwrap()).collect()
        });
    }

    spring_conditions
}

#[test]
fn test_parse() {
    let spring_conditions = parse("test_input");
    assert_eq!(spring_conditions.len(), 6);
}

#[test]
fn test_combinations_indiv() {
    let sp = SpringCondition {
        template: String::from("???.###"),
        group: vec![1,1,3]
    };
    assert_eq!(combination(&sp), 1);
    let sp = SpringCondition {
        template: String::from("?###????????"),
        group: vec![3,2,1]
    };
    assert_eq!(combination(&sp), 10);
}

#[test]
fn test_combinations() {
    let spring_conditions = parse("test_input");
    assert_eq!(combinations(&spring_conditions), 21);
}
