use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_to_string(&args[1]).unwrap();

    println!("Parte 1: {}", res1(&input));
    println!("Parte 2: {}", res2(&input));
}

fn res1(input : &str) -> usize {
    let mut res = 0;

    let mut pos = (0,0);
    let mut last_pos = (0,0);
    for line in input.lines() {
        let info : Vec<&str> = line.split(" ").collect();
        let dir = info[0];
        let len :i32 = info[1].parse().unwrap();

        pos = (if dir == "L" {pos.0-len} else if dir == "R" {pos.0+len} else {pos.0},
               if dir == "U" {pos.1-len} else if dir == "D" {pos.1+len} else {pos.1});

        res += (last_pos.0 * pos.1) - (last_pos.1 * pos.0) + len;

        last_pos = pos;
    }

    (res / 2 + 1) as usize
}

fn res2(input : &str) -> usize {
    let mut res = 0;

    let mut pos = (0,0);
    let mut last_pos = (0,0);
    for line in input.lines() {
        let code : &str = line.split(" ").collect::<Vec<&str>>()[2].trim_matches(|c:char| !c.is_ascii_hexdigit());
        let dir = match code.chars().last().unwrap() {
            '0' => "R",
            '1' => "D",
            '2' => "L",
            '3' => "U",
            _ => ""
        };
        let len_hex = code.strip_suffix(|c:char| ['0','1','2','3'].contains(&c)).unwrap();
        let len = i64::from_str_radix(len_hex, 16).unwrap();

        pos = (if dir == "L" {pos.0-len} else if dir == "R" {pos.0+len} else {pos.0},
               if dir == "U" {pos.1-len} else if dir == "D" {pos.1+len} else {pos.1});

        res += (last_pos.0 * pos.1) - (last_pos.1 * pos.0) + len;

        last_pos = pos;
    }

    (res / 2 + 1) as usize
}
