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

fn min_path_to(from : &str, contains : &str, instructions : &Vec<usize>, nodes : &HashMap<&str, Vec<&str>>) -> u32 {
    let mut i = 0;
    let mut current = from;
    while !current.contains(contains) {
        let instruction_index = i % instructions.len();
        current = nodes.get(current).unwrap()[instructions[instruction_index]];
        i += 1;
    }
    i as u32
}

fn res1(input : &Vec<String>) -> u32 {
    let instructions : Vec<usize> = input[0].chars().map(|c| if c=='R' {1} else {0}).collect();

    let mut nodes = HashMap::new();
    for i in 2..input.len() {
        let s : Vec<&str> = input[i].split(" = ").collect();
        let node = s[0];

        let info : Vec<&str> = (&(s[1])[1..9]).split(", ").collect();

        nodes.insert(node, info);
    }

    min_path_to("AAA", "ZZZ", &instructions, &nodes)
}

fn res2(input : &Vec<String>) -> u64 {
    let instructions : Vec<usize> = input[0].chars().map(|c| if c=='R' {1} else {0}).collect();

    let mut currents : Vec<&str> = vec![];

    let mut nodes = HashMap::new();
    for i in 2..input.len() {
        let s : Vec<&str> = input[i].split(" = ").collect();
        let node = s[0];

        if node.contains("A") {
            currents.push(node);
        }

        let info : Vec<&str> = (&(s[1])[1..9]).split(", ").collect();

        nodes.insert(node, info);
    }

    let dists : Vec<u64> = currents.iter().map(|n| min_path_to(n, "Z", &instructions, &nodes) as u64).collect();
    vec_lcm(&dists)
}

// Euclides
fn gcd(x : u64, y : u64) -> u64 {
    let mut a = x;
    let mut b = y;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(x : u64, y : u64) -> u64 {
    (x * y) / gcd(x, y)
}

fn vec_lcm(nums : &Vec<u64>) -> u64 {
    let mut res = nums[0];
    for n in nums {
        res = lcm(res, *n);
    }
    res
}
