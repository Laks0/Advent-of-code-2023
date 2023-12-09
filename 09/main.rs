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

fn find_next(sec : &Vec<i32>) -> i32 {
    let mut null = true;
    for n in sec {
        null = null && (*n==0);
    }
    if null {
        return 0;
    }

    let mut difs : Vec<i32> = vec![];
    for i in 1..sec.len() {
        difs.push(sec[i] - sec[i-1]);
    }

    sec[sec.len()-1] + find_next(&difs)
}

fn find_prev(sec : &Vec<i32>) -> i32 {
    let mut null = true;
    for n in sec {
        null = null && (*n==0);
    }
    if null {
        return 0;
    }

    let mut difs : Vec<i32> = vec![];
    for i in 1..sec.len() {
        difs.push(sec[i] - sec[i-1]);
    }

    sec[0] - find_prev(&difs)
}

fn res1(input : &Vec<String>) -> i32 {
    let mut res : i32 = 0;

    for line in input {
        let n = find_next(&line.split(" ")
                          .map(|s| s.parse::<i32>().unwrap())
                          .collect::<Vec<i32>>()
                         );
        res += n;
    }

    res
}

fn res2(input : &Vec<String>) -> i32 {
    let mut res : i32 = 0;

    for line in input {
        let n = find_prev(&line.split(" ")
                          .map(|s| s.parse::<i32>().unwrap())
                          .collect::<Vec<i32>>()
                         );
        res += n;
    }

    res
}
