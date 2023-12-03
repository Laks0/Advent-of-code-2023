use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
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

    // Mapa de adyacencias
    let mut adj : Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];
    for y in 0..input.len() {
        let fila : Vec<char> = input[y].chars().collect();
        for x in 0..input[y].len() {
            let digit = fila[x].to_digit(10);
            if digit != None || fila[x] == '.' {
                continue;
            }
            // Es un símbolo
            for i in cmp::max(y-1, 0)..=cmp::min(y+1, input.len()-1) {
                for l in cmp::max(x-1, 0)..=cmp::min(x+1, fila.len()-1) {
                    adj[i][l] = true;
                }
            }
        }
    }

    // Números
    for y in 0..input.len() {
        let fila : Vec<char> = input[y].chars().collect();

        let mut num : u32 = 0;
        let mut es_adj = false;
        for x in 0..input[y].len() {
            let digit = fila[x].to_digit(10);
            if digit == None {
                if es_adj {
                    res += num;
                }
                num = 0;
                es_adj = false;
                continue;
            }

            es_adj = es_adj || adj[y][x];
            num = num * 10 + digit.unwrap();
        }

        if es_adj {
            res += num;
        }
    }

    res
}

fn res2(input : &Vec<String>) -> u32 {
    let mut res : u32 = 0;

    let mut gears = HashMap::new();

    for y in 0..input.len() {
        let fila : Vec<char> = input[y].chars().collect();

        let mut num = 0;
        let mut adj_gears : HashSet<String> = HashSet::new();
        for x in 0..fila.len() {
            let digit = fila[x].to_digit(10);
            if digit == None {
                for p in adj_gears.clone() {
                    // Agrega el número a la entrada de p si existe, la crea si no
                    gears.entry(p).and_modify(|v: &mut Vec<u32>| v.push(num)).or_insert(vec![num]);
                }
                adj_gears.drain();
                num = 0;
                continue;
            }

            num = num * 10 + digit.unwrap();
            let from_y = if y > 0 {y-1} else {0};
            let from_x = if x > 0 {x-1} else {0};
            for i in from_y..=cmp::min(y+1, input.len()-1) {
                for l in from_x..=cmp::min(x+1, fila.len()-1) {
                    let cs : Vec<char> = input[i].chars().collect();
                    if cs[l] == '*' {
                        adj_gears.insert(format!("{i}, {l}"));
                    }
                }
            }
        }
        for p in adj_gears.clone() {
            gears.entry(p).and_modify(|v: &mut Vec<u32>| v.push(num)).or_insert(vec![num]);
        }
    }

    for g in gears {
        let list : Vec<u32> = g.1;
        if list.len() != 2 {
            continue;
        }
        res += list[0] * list[1];
    }

    res
}
