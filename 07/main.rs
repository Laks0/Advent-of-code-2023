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

const ORDER : [char;13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

fn get_card_index(card: char) -> u8 {
    ORDER.iter().position(|&c| c == card).unwrap() as u8
}

#[derive(Debug)]
struct Hand {
    cards : Vec<char>,
    bet : u32
}

impl Hand {
    fn play(&self, with_jokers : bool) -> u8 {
        let mut card_amount : [u8;13] = [0;13];
        let mut js = 0;
        for card in &(self.cards) {
            if *card == 'J' && with_jokers {
                js += 1;
                continue;
            }

            card_amount[get_card_index(*card) as usize] += 1;
        }

        // Cantidad de grupos
        let mut groups_in_size : [u8;6] = [0;6];
        for i in card_amount {
            groups_in_size[i as usize] += 1;
        }

        // Reemplazar jokers
        let mut max_group = 0;
        for i in 0..=5 {
            if groups_in_size[i] > 0 {
                max_group = i;
            }
        }
        if with_jokers {
            groups_in_size[max_group] -= 1;
            groups_in_size[max_group + js] += 1;
        }

        if groups_in_size[5] > 0 {
            return 0; // Five of a kind
        }

        if groups_in_size[4] > 0 {
            return 1; // Four of a kind
        }
        
        if groups_in_size[3] > 0 && groups_in_size[2] > 0 {
            return 2; // Full house
        }

        if groups_in_size[3] > 0 {
            return 3; // Three of a kind
        }
        
        if groups_in_size[2] == 2 {
            return 4; // Two Pair
        }
        
        if groups_in_size[2] == 1 {
            return 5; // Pair
        }

        6 // High card
    }

    fn compare(&self, cmp : &Hand, with_jokers: bool) -> std::cmp::Ordering {
        // Tipos
        if self.play(with_jokers) < cmp.play(with_jokers) {
            return std::cmp::Ordering::Greater;
        }
        if self.play(with_jokers) > cmp.play(with_jokers) {
            return std::cmp::Ordering::Less;
        }

        // Desempate
        for i in 0..=5 {
            let mut a_index = get_card_index(self.cards[i]);
            let mut b_index = get_card_index(cmp.cards[i]);

            if a_index == get_card_index('J') {
                a_index = 100;
            }
            if b_index == get_card_index('J') {
                b_index = 100;
            }

            if a_index < b_index {
                return std::cmp::Ordering::Greater;
            }
            if b_index < a_index {
                return std::cmp::Ordering::Less;
            }
        }
        std::cmp::Ordering::Equal
    }
}

fn res1(input : &Vec<String>) -> u32 {
    let mut res : u32 = 0;

    let mut hands : Vec<Hand> = vec![];
    for line in input {
        let s : Vec<&str> = line.split(" ").collect();
        hands.push(Hand {
            cards: s[0].chars().collect(),
            bet: s[1].parse().unwrap()
        });
    }

    hands.sort_by(|a, b| a.compare(b, false));

    for i in 0..hands.len() {
        res += ((i as u32)+1) * hands[i].bet;
    }

    res
}

fn res2(input : &Vec<String>) -> u32 {
    let mut res : u32 = 0;

    let mut hands : Vec<Hand> = vec![];
    for line in input {
        let s : Vec<&str> = line.split(" ").collect();
        hands.push(Hand {
            cards: s[0].chars().collect(),
            bet: s[1].parse().unwrap()
        });
    }

    hands.sort_by(|a, b| a.compare(b, true));

    for i in 0..hands.len() {
        res += ((i as u32)+1) * hands[i].bet;
    }

    res
}
