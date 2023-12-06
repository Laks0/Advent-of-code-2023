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

fn res1(input : &Vec<String>) -> u32 {
    let mut res : u32 = 1;

    let mut times : Vec<u32> = vec![];
    for s in input[0].split(" ") {
        if s == "Time:" || s == "" {
            continue;
        }
        times.push(s.parse().unwrap());
    }

    let mut distances : Vec<u32> = vec![];
    for s in input[1].split(" ") {
        if s == "Distance:" || s == "" {
            continue;
        }
        distances.push(s.parse().unwrap());
    }

    for i in 0..times.len() {
        let mut ways = 0;
        for t in 0..times[i] {
            if t*(times[i]-t) > distances[i] {
                ways += 1;
            }
        }
        res *= ways;
    }

    res
}

fn res2(input : &Vec<String>) -> u64 {
    let mut res : u64 = 0;

    let mut time : u64 = 0;
    let mut distance : u64 = 0;

    for c in input[0].chars() {
        let digit = c.to_digit(10);
        if digit == None {
            continue;
        }
        time = time * 10 + (digit.unwrap() as u64);
    }

    for c in input[1].chars() {
        let digit = c.to_digit(10);
        if digit == None {
            continue;
        }
        distance = distance * 10 + (digit.unwrap() as u64);
    }

    for t in 0..=time {
        if t * (time - t) > distance {
            res += 1;
        }
    }

    res
}
