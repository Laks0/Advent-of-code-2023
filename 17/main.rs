use std::env;
use std::fs::read_to_string;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_to_string(&args[1]).unwrap();

    println!("Parte 1: {}", res1(&input));
    println!("Parte 2: {}", res2(&input));
}

fn res1(input : &str) -> u32 {
    let map : Vec<Vec<u32>> = input.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect();

    dist((0,0), (map[0].len()-1,map.len()-1), false, &map)
}

fn res2(input : &str) -> u32 {
    let map : Vec<Vec<u32>> = input.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect();

    dist((0,0), (map[0].len()-1,map.len()-1), true, &map)
}

fn adj((x, y): (usize, usize), last_dir: (i32, i32), repeats: usize, ultra: bool, map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    if x == 0 && y == 0 {return vec![(0,1), (1,0)]}

    let height = map.len();
    let width = map[0].len();

    let max_repeats = if ultra {9} else {2};
    let min_repeats = if ultra {3} else {0};

    let mut res = vec![];

    if x > 0 && (last_dir.0 != -1 || repeats < max_repeats) && last_dir.0 != 1 && !(last_dir.0 != -1 && repeats < min_repeats) && !(repeats==0 && x < min_repeats) {
        res.push((x-1, y));
    }
    if x < width-1 && (last_dir.0 != 1 || repeats < max_repeats) && last_dir.0 != -1 && !(last_dir.0 != 1 && repeats < min_repeats) && !(repeats==0 && x > width-min_repeats){
        res.push((x+1, y));
    }
    if y > 0 && (last_dir.1 != -1 || repeats < max_repeats) && last_dir.1 != 1 && !(last_dir.1 != -1 && repeats < min_repeats) && !(repeats==0 && y < min_repeats){
        res.push((x, y-1));
    }
    if y < height-1 && (last_dir.1 != 1 || repeats < max_repeats) && last_dir.1 != -1 && !(last_dir.1 != 1 && repeats < min_repeats) && !(repeats==0 && y > height-min_repeats){
        res.push((x, y+1));
    }

    res
}

fn dir_to (from: (usize, usize), to: (usize, usize)) -> (i32, i32) {
    (if from.0 < to.0 {1} else if from.0 == to.0 {0} else {-1},
     if from.1 < to.1 {1} else if from.1 == to.1 {0} else {-1})
}

fn dir_num(dir: (i32, i32)) -> usize {
    match dir {
        (1, 0) => 0,
        (0,-1) => 1,
        (-1,0) => 2,
        (0, 1) => 3,
        _ => 0
    }
}

// Dijkstra
fn dist((x, y): (usize, usize), to: (usize, usize), ultra: bool, map: &Vec<Vec<u32>>) -> u32 {
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), x, y, (0,0), 0));

    // parent[y][x][dir_num][repeats]
    let mut parent = vec![vec![vec![vec![(usize::MAX, usize::MAX, usize::MAX, usize::MAX);10];4];map[0].len()];map.len()];
    parent[0][0][0][0] = (0,0,0,0);

    while heap.len() > 0 {
        let (Reverse(d), current_x, current_y, dir, repeats) = heap.pop().unwrap();

        for next in adj((current_x, current_y), dir, repeats, ultra, &map) {
            let n_dir = dir_to((current_x, current_y), next);
            let n_repeats = if n_dir == dir {repeats+1} else {0};

            if parent[next.1][next.0][dir_num(n_dir)][n_repeats].0 != usize::MAX {continue}
            if next == to && n_repeats < 3 && ultra {continue}

            let n_dist = d + map[next.1][next.0];
            parent[next.1][next.0][dir_num(n_dir)][n_repeats] = (current_x, current_y, dir_num(dir), repeats);

            if next == to {
                return n_dist;
            }

            heap.push((Reverse(n_dist), next.0, next.1, n_dir, n_repeats));
        }
    }

    u32::MAX
}
