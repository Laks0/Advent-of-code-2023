use std::env;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_lines(&args[1]);

    println!("Parte 1: {}", res1(&input));
    println!("Parte 2: {}", res2(&input));
}

#[derive(Debug)]
struct Entry {
    dst : u64,
    src : u64,
    len : u64
}

impl Entry {
    fn map(&self, num : u64) -> u64 {
        if num < self.src || num - self.src >= self.len {
            return num;
        }
        self.dst + (num - self.src)
    }
}

fn res1(input : &Vec<String>) -> u64 {
    let seeds : Vec<u64> = input[0].split(": ").collect::<Vec<&str>>()[1]
        .split(" ")
        .map(|n : &str| n.parse::<u64>().unwrap()).collect();

    let steps : Vec<Vec<Entry>> = get_steps(input);

    let mut res : u64 = find_location(seeds[0], &steps);
    for seed in seeds {
        let loc = find_location(seed, &steps);
        res = res.min(loc);
    }
    res
}

#[derive(Debug)]
struct Range {
    start : u64,
    len : u64
}

impl Range {
    fn has_inside(&self, n : u64) -> bool {
        !(n < self.start || n - self.start >= self.len)
    }

    fn intersection(&self, r : &Range) -> Range{
        if self.has_inside(r.start) {
            return Range {
                start: r.start,
                len: r.len.min(self.len - (r.start - self.start))
            };
        }
        if r.has_inside(self.start) {
            return Range {
                start: self.start,
                len: self.len.min(r.len - (self.start - r.start))
            };
        }
        Range {start:0, len:0}
    }

    fn split_intersection(&self, splitter : &Vec<Range>) -> Vec<Range> {
        let mut res : Vec<Range> = vec![];
        for r in splitter {
            let int = self.intersection(r);
            if int.len > 0 {
                res.push(int);
            }
        }

        let mut length_left = self.len;
        let mut current_start = self.start;
        while length_left > 0 {
            let mut covered = false;
            let mut min_next_start = self.start + self.len;
            for i in 0..res.len() {
                let r = &res[i];
                if r.has_inside(current_start) {
                    covered = true;
                    current_start = r.start + r.len;
                    length_left -= r.len;
                    break;
                }
                if r.start > current_start {
                    min_next_start = min_next_start.min(r.start);
                }
            }

            if !covered {
                let new_len = min_next_start - current_start;
                res.push(Range{start: current_start, len : new_len});
                length_left -= new_len;
                current_start += new_len;
            }
        }

        res
    }
}

fn res2(input : &Vec<String>) -> u64 {
    let steps : Vec<Vec<Entry>> = get_steps(input);

    let seeds : Vec<u64> = input[0].split(": ").collect::<Vec<&str>>()[1]
        .split(" ")
        .map(|n : &str| n.parse::<u64>().unwrap()).collect();

    let mut seed_ranges : Vec<Range> = vec![];
    for i in 0..seeds.len() {
        if i % 2 == 1 {
            continue;
        }
        seed_ranges.push(Range {start: seeds[i], len: seeds[i+1]});
    }

    let mut step_ranges : Vec<Range> = seed_ranges;
    for step in steps {
        let mut new_ranges : Vec<Range> = vec![];

        let src_ranges : Vec<Range> = step.iter()
            .map(|e : &Entry| Range {start : e.src, len: e.len})
            .collect();

        for r in step_ranges {
            for s in r.split_intersection(&src_ranges) {
                new_ranges.push(Range {start: step_translate(s.start, &step), len: s.len});
            }
        }

        step_ranges = new_ranges;
    }

    let mut res : u64 = step_ranges[0].start;
    for s in step_ranges {
        res = res.min(s.start);
    }

    res
}

fn find_location(seed : u64, steps : &Vec<Vec<Entry>>) -> u64 {
    let mut num = seed;

    for step in steps {
        num = step_translate(num, &step);
    }

    num
}

fn step_translate(num : u64, step: &Vec<Entry>) -> u64 {
    let mut n = num;
    for entry in step {
        if n == entry.map(n) {
            continue;
        }
        n = entry.map(n);
        break;
    }
    n
}

fn get_steps(input : &Vec<String>) -> Vec<Vec<Entry>> {
    let mut steps : Vec<Vec<Entry>> = vec![vec![]];
    for i in 2..input.len() {
        let line = &input[i];
        if line == "" {
            steps.push(vec![]);
            continue;
        }

        let words : Vec<&str> = line.split(" ").collect();
        if words[1] == "map:" {
            continue;
        }

        let params : Vec<u64> = line.split(" ")
            .map(|n : &str| n.parse::<u64>().unwrap()).collect();
        let entry = Entry {src: params[1], dst : params[0], len : params[2]};
        let last = steps.len()-1;
        steps[last].push(entry);
    }
    steps
}
