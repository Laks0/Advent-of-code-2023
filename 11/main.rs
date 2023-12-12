use std::env;
use std::fs::read_to_string;
use std::cmp::max;
use std::cmp::min;

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

fn res1(input : &Vec<String>) -> u64 {
    get_distances(input, 2)
}

fn res2(input : &Vec<String>) -> u64 {
    get_distances(input, 1000000)
}

fn get_distances(input: &Vec<String>, expansion: u64) -> u64 {
    let mut res : u64 = 0;

    let map : Vec<Vec<char>> = input.iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut stars : Vec<(usize, usize)> = vec![];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '#' {
                stars.push((x,y));
            }
        }
    }

    let empty_line : Vec<bool> = map.iter()
        .map(|line| *line == vec!['.';map[0].len()])
        .collect();

    let mut empty_col : Vec<bool> = vec![];
    for x in 0..map[0].len() {
        let mut empty = true;
        for y in 0..map.len() {
            empty = empty && map[y][x] == '.';
        }
        empty_col.push(empty);
    }

    let mut counted : Vec<bool> = vec![false;stars.len()];
    for i in 0..stars.len() {
        counted[i] = true;
        let from = stars[i];

        for j in 0..stars.len() {
            if counted[j] {continue}

            let to = stars[j];
            let max_x = max(to.0, from.0);
            let min_x = min(to.0, from.0);
            let max_y = max(to.1, from.1);
            let min_y = min(to.1, from.1);

            let mut dist = ((max_x - min_x) + (max_y - min_y)) as u64;

            for x in min_x..max_x {dist += if empty_col[x]  {expansion-1} else {0}}
            for y in min_y..max_y {dist += if empty_line[y] {expansion-1} else {0}}

            res += dist;
        }
    }

    res
}
