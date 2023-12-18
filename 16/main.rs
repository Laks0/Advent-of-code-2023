use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_to_string(&args[1]).unwrap();

    println!("Parte 1: {}", res1(&input));
    println!("Parte 2: {}", res2(&input));
}

fn res1(input : &str) -> u32 {
    let map : Vec<Vec<char>> = input.lines().map(|x| x.chars().collect::<Vec<char>>()).collect();

    get_cells_energized((0,0), (1,0), &map)
}

fn res2(input : &str) -> u32 {
    let map : Vec<Vec<char>> = input.lines().map(|x| x.chars().collect::<Vec<char>>()).collect();

    let mut res = 0;

    let width = map[0].len();
    let height = map.len();

    for x in 0..width {
        res = res
            .max(get_cells_energized((x,0), (0,1), &map))
            .max(get_cells_energized((x,height-1), (0,-1), &map));
    }

    for y in 0..height {
        res = res
            .max(get_cells_energized((0,y), (1,0), &map))
            .max(get_cells_energized((width-1,y), (-1,0), &map));
    }

    res
}

fn get_cells_energized(pos : (usize, usize), dir : (i32, i32), map: &Vec<Vec<char>>) -> u32 {
    let mut visited = vec![vec![false;4]; map[0].len()*map.len()];
    energize_from(pos, dir, &map, &mut visited);

    visited.iter().map(|pos| if pos[0] || pos[1] || pos[2] || pos[3] {1} else {0}).sum::<u32>()
}

fn rotate_90((x,y) : (i32, i32)) -> (i32, i32) {
    (y, -x)
}
fn rotate_neg90((x,y) : (i32, i32)) -> (i32, i32) {
    (-y, x)
}

// Para guardar en el cache, necesitamos una id para la dirección
// 0 = este, aumenta a contrareloj
fn get_dir_num(dir : (i32, i32)) -> usize {
    match dir {
        ( 1, 0) => 0,
        ( 0,-1) => 1,
        (-1, 0) => 2,
        ( 0, 1) => 3,
        _ => 0
    }
}

fn energize_from(pos : (usize, usize), dir : (i32, i32), map: &Vec<Vec<char>>, mut visited: &mut Vec<Vec<bool>>) {
    let width = map[0].len();
    let height = map.len();

    let (x, y) = pos;
    let dir_num = get_dir_num(dir);
    let pos_num = x + y*width;

    if visited[pos_num][dir_num] {return}
    visited[pos_num][dir_num] = true;

    let next_dirs : Vec<(i32, i32)> = match map[y][x] {
        '.'  => vec![dir],

        '/'  => vec![if dir.0 == 0 {rotate_neg90(dir)} else {rotate_90(dir)}],
        '\\' => vec![if dir.1 == 0 {rotate_neg90(dir)} else {rotate_90(dir)}],

        '-' if dir.0 != 0 => vec![dir],
        '-'  => vec![(-1, 0), (1, 0)],

        '|' if dir.1 != 0 => vec![dir],
        '|' => vec![(0, -1), (0, 1)],

        _ => vec![]
    };

    for next in next_dirs {
        if (x == 0 && next.0 == -1) || (y == 0 && next.1 == -1)
            || (x == width-1 && next.0 == 1) || (y == height-1 && next.1 == 1) {
            continue;
        }

        let new_pos = ((x as i32 + next.0) as usize, (y as i32 + next.1) as usize);
        energize_from(new_pos, next, &map, &mut visited);
    }
}
