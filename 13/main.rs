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
    let mut res : u32 = 0;

    let pattern_lines : Vec<Vec<String>> = input.split(|x| x == "").map(|x| x.to_vec()).collect();
    let pattern_cols = get_cols(&pattern_lines);

    for i in 0..pattern_lines.len() {
        let col_r = find_reflection(&pattern_cols[i]);
        if col_r != 0 {
            res += col_r;
            continue;
        }

        res += find_reflection(&pattern_lines[i]) * 100;
    }

    res
}

fn res2(input : &Vec<String>) -> u32 {
    let mut res : u32 = 0;

    let pattern_lines : Vec<Vec<String>> = input.split(|x| x == "").map(|x| x.to_vec()).collect();
    let pattern_cols = get_cols(&pattern_lines);

    for i in 0..pattern_lines.len() {
        let col_r = find_reflection_smudged(&pattern_cols[i]);
        if col_r != 0 {
            res += col_r;
            continue;
        }

        res += find_reflection_smudged(&pattern_lines[i]) * 100;
    }

    res
}

fn get_cols(lines: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut pattern_cols : Vec<Vec<String>> = vec![];

    for pattern in lines {
        let width = pattern[0].len();
        let mut cols : Vec<String> = vec![];
        for i in 0..width {
            let col : String = pattern.iter()
                .map(|s : &String | s.chars().nth(i).unwrap())
                .collect();

            cols.push(col);
        }
        pattern_cols.push(cols);
    }

    pattern_cols
}

fn find_reflection(lines: &Vec<String>) -> u32 {
    for i in 0..lines.len()-1 {
        if lines[i] != lines[i+1] {continue}

        let mut valid = true;
        for j in 0..i+1 {
            if i+1+j >= lines.len() {break}

            valid = valid && lines[i-j] == lines[i+1+j];
            if !valid {break}
        }

        if !valid {continue}
        return (i+1) as u32;
    }

    0
}

fn find_reflection_smudged(lines: &Vec<String>) -> u32 {
    for i in 0..lines.len()-1 {
        let mut used_smudge = false;

        let mut valid = true;
        for j in 0..i+1 {
            if i+1+j >= lines.len() {break}

            if lines[i-j] == lines[i+1+j] {continue}

            let mut diff = 0;
            for n in 0..lines[i-j].len() {
                if lines[i-j].chars().nth(n).unwrap() != lines[i+1+j].chars().nth(n).unwrap() {
                    diff += 1;
                }

                // No cambia si diff es 2 o más
                if diff > 1 {break}
            }

            if diff > 1 || used_smudge {
                valid = false;
                break;
            }

            used_smudge = true;
        }

        // Si no usamos el smudge encontramos la línea vieja
        if !valid || !used_smudge {continue}
        return (i+1) as u32;
    }

    0
}
