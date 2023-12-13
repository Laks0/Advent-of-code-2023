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

fn res1(input : &Vec<String>) -> u64 {
    let mut res : u64 = 0;

    for line in input {
        let s : Vec<&str> = line.split(" ").collect();

        let list : Vec<char> = s[0].chars().collect();
        let groups : Vec<u64> = s[1].split(",")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let mut mem = vec![
            vec![
                vec![u64::MAX;list.len()+1]
                ;groups.len()+1]
            ;list.len()+1];
        let p = possibilities(&list, &groups, 0, 0, 0, &mut mem);
        res += p;
    }

    res
}

fn res2(input : &Vec<String>) -> u64 {
    let mut res : u64 = 0;

    for line in input {
        let s : Vec<&str> = line.split(" ").collect();

        let list : Vec<char> = [s[0];5].join("?").chars().collect();
        let groups : Vec<u64> = [s[1];5].join(",").split(",")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let mut mem = vec![
            vec![
                vec![u64::MAX;list.len()+1]
                ;groups.len()+1]
            ;list.len()+1];
        let p = possibilities(&list, &groups, 0, 0, 0, &mut mem);
        res += p;
    }

    res
}

// Programación dinámica
fn possibilities(list: &Vec<char>, groups: &Vec<u64>, i: usize, group_id: usize, current_size: u64, mut mem: &mut Vec<Vec<Vec<u64>>>) -> u64 {
    if mem[i][group_id][current_size as usize] != u64::MAX {
        return mem[i][group_id][current_size as usize];
    }

    if i == list.len() {
        if group_id == groups.len()-1 && current_size != groups[group_id] {
            return 0;
        }
        if group_id < groups.len()-1 {
            return 0;
        }
        if group_id == groups.len() && current_size > 0 {
            return 0;
        }
        return 1;
    }

    if group_id == groups.len() && current_size > 0 {
        return 0;
    }

    let res = match list[i] {
        '.' if current_size == 0 => possibilities(&list, &groups, i+1, group_id, 0, &mut mem),

        '.' | '?' if current_size > 0 && current_size == groups[group_id] => possibilities(&list, &groups, i+1, group_id+1, 0, &mut mem),

        '#' => possibilities(&list, &groups, i+1, group_id, current_size+1, &mut mem),

        // Roto sí o sí
        '?' if 0 < current_size => possibilities(&list, &groups, i+1, group_id, current_size+1, &mut mem),

        '?' => possibilities(&list, &groups, i+1, group_id, 0, &mut mem) + possibilities(&list, &groups, i+1, group_id, current_size+1, &mut mem),

        _ => 0
    };

    mem[i][group_id][current_size as usize] = res;

    res
}
