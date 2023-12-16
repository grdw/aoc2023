use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct SpringCondition {
    template: String,
    group: Vec<u8>
}

impl SpringCondition {
    fn multiply(&mut self) {
        let mut n = vec![];
        let mut p = vec![];
        for _ in 0..5 {
            n.push(self.template.clone());

            for n in &self.group {
                p.push(*n);
            }
        }

        self.template = n.join("?");
        self.group = p;
    }
}

fn main() {
    let mut spring_conditions = parse("input");
    println!("p1: {}", combinations(&spring_conditions));
    for s in spring_conditions.iter_mut() {
        s.multiply();
    }
    println!("p2: {}", combinations(&spring_conditions));
}

fn combination(s: &SpringCondition) -> u32 {
    let mut t = 0;
    let mut dot_group = vec![0];
    for _ in 0..(s.group.len() - 1) {
        dot_group.push(1);
    }

    dot_group.push(0);
    let total_chars = dot_group.iter().sum::<u8>() +
        s.group.iter().sum::<u8>();

    let total = s.template.len() - total_chars as usize;
    let num_sockets = dot_group.len();
    let mut combination = vec![0; num_sockets];
    let last_index = num_sockets - 1;

    println!("{} {} {}", s.template, last_index, total);
    loop {
        // Process the current combination
        if combination.iter().sum::<usize>() == total {
            let mut g = String::new();
            let mut i = 0;

            while i < dot_group.len() {
                g.push_str(&".".repeat(combination[i] + dot_group[i] as usize));
                if let Some(n) = s.group.get(i) {
                    g.push_str(&"#".repeat(*n as usize));
                }

                i+=1;
            }

            let mut m = true;
            for (i, c) in s.template.chars().enumerate() {
                if c == '?' {
                    continue
                }

                if g.chars().nth(i) != Some(c) {
                    m = false;
                    break;
                }
            }
            if m {
                t += 1;
            }
        }

        // Find the rightmost element that can be incremented
        let mut i = last_index;
        while i > 0 && (combination[i] == total) {
            i -= 1;
        }

        // Increment the rightmost element
        combination[i] += 1;

        for j in (i + 1)..num_sockets {
            combination[j] = 0;
        }

        if combination[0] > total {
            break;
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
        template: String::from("?###????????"),
        group: vec![3,2,1]
    };
    assert_eq!(combination(&sp), 10);
    let sp = SpringCondition {
        template: String::from("???.###"),
        group: vec![1,1,3]
    };
    assert_eq!(combination(&sp), 1);
}

#[test]
fn test_combinations() {
    let spring_conditions = parse("test_input");
    assert_eq!(combinations(&spring_conditions), 21);
}
