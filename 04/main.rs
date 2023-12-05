use std::env;
use std::fs::read_to_string;
use std::collections::HashSet;
use std::cmp;

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
    let mut res : u32 = 0;

    for card in input {
        let info : Vec<&str> = card.split(": ").collect::<Vec<&str>>()[1]
            .split(" | ").collect();

        let mut winners = HashSet::new();
        for winner_number in info[0].split(" ") {
            if winner_number == "" {
                continue;
            }
            winners.insert(winner_number);
        }

        let mut points = 0;
        for card_number in info[1].split(" ") {
            if card_number == "" {
                continue;
            }

            if !winners.contains(card_number) {
                continue;
            }

            if points > 0 {
                points *= 2;
                continue;
            }
            points = 1;
        }

        res += points;
    }

    res
}

fn res2(input : &Vec<String>) -> u32 {
    let mut res : u32 = 0;

    let mut copies = vec![1;input.len()];

    for i in 0..input.len() {
        res += copies[i];

        let points = res1(&(vec![String::from(&input[i])]));
        if i+1 >= input.len() || points == 0 {
            continue;
        }

        let matches = (points as f64).log2() + 1.0;
        for j in i+1..=cmp::min(i+(matches as usize), input.len() - 1) {
            copies[j] += copies[i];
        }
    }

    res
}
