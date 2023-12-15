use std::env;
use std::fs::read_to_string;
use std::collections::HashMap;

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

    let mut map : Vec<Vec<char>> = input.iter()
        .map(|x| x.chars().collect())
        .collect();

    roll_north(&mut map);

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                res += (map.len() - y) as u32;
            }
        }
    }
    res
}

fn res2(input : &Vec<String>) -> u32 {
    let mut res : u32 = 0;

    let mut map : Vec<Vec<char>> = input.iter()
        .map(|x| x.chars().collect())
        .collect();

    let mut iterations = HashMap::new();
    let mut i = 0;
    while i < 1000000000 {
        roll_north(&mut map);
        roll_west(&mut map);
        roll_south(&mut map);
        roll_east(&mut map);

        let full_string : String = map.iter()
            .map(|x| x.iter().collect::<String>()).collect::<Vec<String>>()
            .join("");

        if iterations.contains_key(&full_string) {
            let last_start = iterations.get(&full_string).unwrap();
            let cycle_lenght : usize = i - last_start;

            let cycles_left : usize = ( 1000000000 - i ) / cycle_lenght;

            i += cycle_lenght * cycles_left;
        } else {
            iterations.insert(full_string, i);
        }

        i+=1;
    }

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                res += (map.len() - y) as u32;
            }
        }
    }

    res
}

fn roll_north(map : &mut Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] != 'O' {continue}

            let mut i = y;

            while i > 0 && map[i-1][x] == '.' {
                map[i][x] = '.';
                i -= 1;
            }

            map[i][x] = 'O';
        }
    }
}

fn roll_south(map : &mut Vec<Vec<char>>) {
    for y in (0..map.len()).rev() {
        for x in 0..map[0].len() {
            if map[y][x] != 'O' {continue}

            let mut i = y;

            while i < map.len()-1 && map[i+1][x] == '.' {
                map[i][x] = '.';
                i += 1;
            }

            map[i][x] = 'O';
        }
    }
}

fn roll_west(map : &mut Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] != 'O' {continue}

            let mut i = x;

            while i > 0 && map[y][i-1] == '.' {
                map[y][i] = '.';
                i -= 1;
            }

            map[y][i] = 'O';
        }
    }
}

fn roll_east(map : &mut Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in ( 0..map[0].len() ).rev() {
            if map[y][x] != 'O' {continue}

            let mut i = x;

            while i < map[0].len()-1 && map[y][i+1] == '.' {
                map[y][i] = '.';
                i += 1;
            }

            map[y][i] = 'O';
        }
    }
}
