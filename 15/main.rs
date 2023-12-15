use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input : String = read_to_string(&args[1]).unwrap();

    println!("Parte 1: {}", res1(&input));
    println!("Parte 2: {}", res2(&input));
}

fn res1(input : &str) -> u32 {
    input.split(",").map(get_hash).sum::<u32>()
}

fn res2(input : &str) -> u32 {
    let mut res : u32 = 0;

    let mut boxes :Vec<Vec<(&str, &str)>> = vec![vec![];256];

    for seq in input.split(",") {
        let (label, focus) = seq.trim().split_once(|x| x=='=' || x=='-').unwrap();
        let n = get_hash(label) as usize;

        let operator = seq.trim_matches(|x:char| !x.is_ascii_punctuation());

        match operator {
            "=" => {
                let search = boxes[n].iter().position(|x| x.0 == label);

                match search {
                    Some(i) => {boxes[n][i] = (label, focus);}
                    None => {boxes[n].push((label, focus));}
                }
            },
            "-" => {
                let search = boxes[n].iter().position(|x| x.0 == label);
                if let Some(i) = search {boxes[n].remove(i);}
            }
            _ => {}
        }
    }

    for (n, b) in boxes.iter().enumerate() {
        for (i, (_, focus)) in b.iter().enumerate() {
            res += (n+1) as u32 * (i+1) as u32 * focus.parse::<u32>().unwrap();
        }
    }

    res
}

fn get_hash(input : &str) -> u32 {
    let mut val : u32 = 0;
    for c in input.trim().as_bytes() {
        val += *c as u32;
        val = (val * 17)%256;
    }
    val
}
