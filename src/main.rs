use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        let mut after_swapping = swap_chars_in_word(&line, 0.05);
        //String::from(line).split(' ').for_each(|x| after_swapping.push_str(&swap_chars_in_word(&x, 0.05)).push_str(" "));
        
        // handle character substitutions
        let mut line: Vec<char> = after_swapping.chars().collect();
        line.iter_mut().for_each(|mut c| make_char_substitutions(&mut c));
        println!("{}", line.iter().collect::<String>());
    }
}

fn make_char_substitutions(c: &mut char) -> () {
    //generate a random vowel
    let random_v : char = vec!{'a', 'e', 'i', 'o', 'u'}[(rand::random::<f64>() / 0.2 ) as usize];
    match c {
        '0' => switch_character_probabilistic(c, '⁰', 0.40),
        '1' => switch_character_probabilistic(c, '¹', 0.40),
        '2' => switch_character_probabilistic(c, '²', 0.40),
        '3' => switch_character_probabilistic(c, '³', 0.40),
        '/' => switch_character_probabilistic(c, 'ᐟ', 0.40),
        'a' => switch_character_probabilistic(c, random_v, 0.20),
        'e' => switch_character_probabilistic(c, random_v, 0.20),
        'i' => switch_character_probabilistic(c, random_v, 0.20),
        'o' => switch_character_probabilistic(c, random_v, 0.20),
        'E' => switch_character_probabilistic(c, '€', 0.20),
        'u' => switch_character_probabilistic(c, '7', 0.10),
        'y' => switch_character_probabilistic(c, 'λ', 0.10),
        'p' => switch_character_probabilistic(c, 'b', 0.10),
        'b' => switch_character_probabilistic(c, 'p', 0.10),
        'h' => switch_character_probabilistic(c, 'j', 0.10),
        'd' => switch_character_probabilistic(c, 'n', 0.10),
        'l' => if get_random(0.50) {*c = 'I';} else {*c = 'i';}

        _   => switch_character_probabilistic(c, c.to_ascii_uppercase(), 0.05)

    }
}

fn switch_character_probabilistic(to_switch: &mut char, new: char, prob:f64) -> () {
    if get_random(prob) {
        *to_switch = new;
    }
}

// there's probably a better way to do this
// https://stackoverflow.com/questions/61683658/how-to-change-order-of-letter-in-rust-with-swap
fn swap_chars_in_word(input: &str, prob_per_pair: f64) -> String {
    let mut word: Vec<char> = input.chars().collect();
    if input.len() <= 1 {
        return String::from(input);
    }
    for pair in word.chunks_mut(2) {
        if pair.len() == 2 && get_random(prob_per_pair) {
            pair.swap(0,1);
        }
    }
    word.iter().collect::<String>()
}

fn get_random(prob: f64) -> bool {
    (rand::random::<f64>() / prob ) as i32 == 0
}