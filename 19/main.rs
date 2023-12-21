use std::env;
use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_to_string(&args[1]).unwrap();

    println!("Parte 1: {}", res1(&input));
    println!("Parte 2: {}", res2(&input));
}

struct Workflow {
    rules : Vec<Rule>
}

#[derive(Debug)]
struct Rule {
    automatic : bool,
    cat : char,
    gt : bool, // true = ">", false = "<"
    num : u64,
    result : String
}

impl From<&str> for Workflow {
    fn from(s: &str) -> Workflow {
        let (_, rs) = s.split_once("{").unwrap();
        Workflow {
            rules: rs.trim_end_matches("}")
                .split(",")
                .map(|r : &str| Rule::from(r))
                .collect()
        }
    }
}

impl Workflow {
    fn next(&self, part: &(u64, u64, u64, u64)) -> String {
        for rule in &self.rules {
            if !rule.passes(&part) {continue}
            return rule.result.clone();
        }
        "R".to_owned()
    }
}

impl From<&str> for Rule {
    fn from(s: &str) -> Rule {
        if !s.contains(":") {
            return Rule {
                automatic: true,
                cat: '_', gt: false, num: 0,
                result: s.to_owned()
            };
        }

        Rule {
            automatic: false,
            cat: s.chars().next().unwrap(),
            gt: s.contains(">"),
            num: s.trim_matches(|c: char| !c.is_numeric()).parse().unwrap(),
            result: s.split_once(":").unwrap().1.to_owned()
        }
    }
}

impl Rule {
    fn passes(&self, part: &(u64, u64, u64, u64)) -> bool {
        if self.automatic {return true}

        let part_cat = match self.cat {
            'x' => part.0,
            'm' => part.1,
            'a' => part.2,
            's' => part.3,
            _ => 0,
        };

        (self.gt && part_cat > self.num) || (!self.gt && part_cat < self.num)
    }
}

fn res1(input: &str) -> u64 {
    let (workflows_str, parts_str) = input.split_once("\n\n").unwrap();

    let mut workflows = HashMap::new();
    for s in workflows_str.lines() {
        workflows.insert(s.split_once("{").unwrap().0.to_owned(), Workflow::from(s));
    }

    let parts: Vec<(u64, u64, u64, u64)> = parts_str.lines()
        .map(|s: &str| {
            let part_vec: Vec<u64> = s.split(",")
                .map(|s: &str| s.trim_matches(|c: char| !c.is_numeric()).parse::<u64>().unwrap())
                .collect();

            (part_vec[0], part_vec[1], part_vec[2], part_vec[3])
        }).collect();

    let mut res = 0;

    for part in parts {
        let mut current = "in".to_owned();
        while current != "A" && current != "R" {
            current = workflows.get(&current).unwrap().next(&part);
        }
        if current == "A" {res += part.0 + part.1 + part.2 + part.3}
    }

    res
}

fn res2(input: &str) -> u64 {
    let (workflows_str, _) = input.split_once("\n\n").unwrap();

    let mut workflows = HashMap::new();
    for s in workflows_str.lines() {
        workflows.insert(s.split_once("{").unwrap().0.to_owned(), Workflow::from(s));
    }

    let mut res = 0;

    let mut queue = VecDeque::from(vec![("in".to_owned(), (1, 4000), (1, 4000), (1, 4000), (1, 4000))]);

    while let Some((curr, mut x, mut m, mut a, mut s)) = queue.pop_back() {
        if curr == "R" {continue}
        if curr == "A" {res += (x.1-x.0+1) * (m.1-m.0+1) * (a.1-a.0+1) * (s.1-s.0+1); continue;}

        for rule in &workflows.get(&curr).unwrap().rules {
            if rule.automatic {queue.push_back((rule.result.to_owned(), x, m, a, s)); continue;}

            let relevant_interval = match rule.cat {
                'x' => {x},
                'm' => {m},
                'a' => {a},
                's' => {s},
                _ => {(0,0)}
            };

            // Pasa todo el intervalo
            if (rule.gt && relevant_interval.0 > rule.num) || (!rule.gt && relevant_interval.1 < rule.num) {
                queue.push_back((rule.result.to_owned(), x, m, a, s));
                break;
            }
            // No pasa nada
            if (rule.gt && relevant_interval.1 < rule.num) || (!rule.gt && relevant_interval.0 > rule.num) {
                continue;
            }

            // Intervalo relevante que pasa la condición
            let pass_interval = if rule.gt {(rule.num + 1, relevant_interval.1)} else {(relevant_interval.0, rule.num-1)};
            // Intervalo que no pasa
            let miss_interval = if rule.gt {(relevant_interval.0, rule.num)} else {(rule.num, relevant_interval.1)};

            match rule.cat {
                'x' => {
                    queue.push_back((rule.result.to_owned(), pass_interval, m, a, s));
                    x = miss_interval;
                },
                'm' => {
                    queue.push_back((rule.result.to_owned(), x, pass_interval, a, s));
                    m = miss_interval;
                },
                'a' => {
                    queue.push_back((rule.result.to_owned(), x, m, pass_interval, s));
                    a = miss_interval;
                },
                's' => {
                    queue.push_back((rule.result.to_owned(), x, m, a, pass_interval));
                    s = miss_interval;
                },
                _ => {}
            }
        }
    }

    res
}
