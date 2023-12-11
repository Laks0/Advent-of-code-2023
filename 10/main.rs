use std::env;
use std::fs::read_to_string;
use std::collections::VecDeque;

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

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Pos {
    x : usize,
    y : usize
}

fn res1(input : &Vec<String>) -> u32 {
    let mut start = Pos{x:0, y:0};

    let map : Vec<Vec<char>> = input.iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                start = Pos {x: x, y: y};
            }
        }
    }

    let dists : Vec<Vec<i32>> = bfs(&map, start);

    let mut res = 0;
    for line in dists {
        for n in line {
            res = res.max(n);
        }
    }

    res as u32
}

// Representación con grafos:
//  * Cada punto es un vértice
//  * Los vértices adyacentes son los que se conectan con la casilla
fn adj(p : Pos, map : &Vec<Vec<char>>) -> Vec<Pos> {
    let width = map[0].len();
    let height = map.len();

    let mut res : Vec<Pos> = vec![];
    if p.x > 0 {
        res.push(Pos {x: p.x-1, y: p.y});
    }
    if p.x < width - 1 {
        res.push(Pos {x: p.x+1, y: p.y});
    }
    if p.y > 0 {
        res.push(Pos {x: p.x, y: p.y-1});
    }
    if p.y < height -1 {
        res.push(Pos {x: p.x, y: p.y+1});
    }

    if map[p.y][p.x] == 'S' {
        res.retain(|a| adj(*a, &map).contains(&p) && map[a.y][a.x] != '.');
    }

    match map[p.y][p.x] {
        '|' => res.retain(|a| a.x == p.x),
        '-' => res.retain(|a| a.y == p.y),

        'L' if p.y > 0 => res.retain(|a| *a == Pos {x: p.x + 1, y: p.y} || *a == Pos{x: p.x, y: p.y - 1}),
        'L' => res.retain(|a| *a == Pos {x: p.x + 1, y: p.y}),

        'J' if p.x > 0 && p.y > 0 => res.retain(|a| *a == Pos {x: p.x - 1, y: p.y} || *a == Pos{x: p.x, y: p.y - 1}),
        'J' if p.x > 0 => res.retain(|a| *a == Pos {x: p.x - 1, y: p.y}),
        'J' if p.y > 0 => res.retain(|a| *a == Pos{x: p.x, y: p.y - 1}),
        'J' => res = vec![],

        '7' if p.x > 0 => res.retain(|a| *a == Pos {x: p.x - 1, y: p.y} || *a == Pos{x: p.x, y: p.y + 1}),
        '7' => res.retain(|a| *a == Pos{x: p.x, y: p.y + 1}),

        'F' => res.retain(|a| *a == Pos {x: p.x + 1, y: p.y} || *a == Pos{x: p.x, y: p.y + 1}),

        _ => {}
    }

    res
}

fn bfs(map : &Vec<Vec<char>>, start : Pos) -> Vec<Vec<i32>> {
    let mut dists : Vec<Vec<i32>> = vec![vec![-1;map[0].len()]; map.len()];
    dists[start.y][start.x] = 0;

    let mut frontier = VecDeque::new();
    frontier.push_back(start);
    while frontier.len() > 0 {
        let current = frontier.pop_front().unwrap();

        for n in adj(current, &map) {
            if dists[n.y][n.x] != -1 {
                continue;
            }
            dists[n.y][n.x] = dists[current.y][current.x] + 1;
            frontier.push_back(n);
        }
    }

    dists
}

fn res2(input : &Vec<String>) -> u32 {
    let mut res : u32 = 0;

    let mut start = Pos{x:0, y:0};

    let map : Vec<Vec<char>> = input.iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                start = Pos {x: x, y: y};
            }
        }
    }

    let dists : Vec<Vec<i32>> = bfs(&map, start);

    // Atraviezo de izquierda a derecha, alternando si está adentro o no del loop cuando veo una
    // pipa del loop que apunta al norte
    for y in 0..map.len() {
        let mut inside = false;
        for x in 0..map[0].len() {
            if dists[y][x] != -1 { // loop pipe
                match map[y][x] {
                    'L' | '|' | 'J' => inside = !inside,
                    'S' if y > 0 && adj(Pos{x: x, y:y}, &map).contains(&Pos{x:x,y:y-1}) => inside = !inside,
                    _ => continue,
                }
                continue;
            }

            if inside {
                res += 1;
            }
        }
    }

    res
}
