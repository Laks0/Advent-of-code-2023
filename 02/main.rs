use std::cmp;
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

fn res1(input : &Vec<String>) -> i16 {
    let mut res : i16 = 0;

    for id in 0..input.len() {
        // Valores máximos
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        let ronda : &str = input[id].split(": ").collect::<Vec<&str>>()[1];

        for turno in ronda.split("; ") {
            for columna in turno.split(", ") {
                let info : Vec<&str> = columna.split(" ").collect();
                if info[1] == "red" {
                    r = cmp::max(r, info[0].parse().unwrap());
                }
                if info[1] == "green" {
                    g = cmp::max(g, info[0].parse().unwrap());
                }
                if info[1] == "blue" {
                    b = cmp::max(b, info[0].parse().unwrap());
                }
            }
        }

        if r <= 12 && g <= 13 && b <= 14 {
            res += (id as i16) + 1;
        }
    }

    res
}

fn res2(input : &Vec<String>) -> u32 {
    let mut res : u32 = 0;

    for id in 0..input.len() {
        // Valores máximos
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        let ronda : &str = input[id].split(": ").collect::<Vec<&str>>()[1];

        for turno in ronda.split("; ") {
            for columna in turno.split(", ") {
                let info : Vec<&str> = columna.split(" ").collect();
                if info[1] == "red" {
                    r = cmp::max(r, info[0].parse().unwrap());
                }
                if info[1] == "green" {
                    g = cmp::max(g, info[0].parse().unwrap());
                }
                if info[1] == "blue" {
                    b = cmp::max(b, info[0].parse().unwrap());
                }
            }
        }

        res += r * g * b;
    }

    res
}
