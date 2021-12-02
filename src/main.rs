use anyhow::Result;
use std::io;
use std::io::prelude::*;

fn main() -> Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // randomly transpose characters in text
        let after_swapping = swap_chars_in_word(&line?, 0.05);

        // handle rule-based character substitutions
        let after_subst = do_substitutions(after_swapping)?;
        //output
        println!("{}", after_subst);
    }
    Ok(())
}

fn do_substitutions(input: String) -> Result<String> {
    let input = replace_prob(input, ("0", "⁰"), 0.40)?;
    let input = replace_prob(input, ("1", "¹"), 0.40)?;
    let input = replace_prob(input, ("2", "²"), 0.40)?;
    let input = replace_prob(input, ("3", "³"), 0.40)?;
    let input = replace_prob(input, ("/", "ᐟ"), 0.40)?;
    let input = replace_prob(input, ("a", &random_vowel_subst()), 0.20)?;
    let input = replace_prob(input, ("e", &random_vowel_subst()), 0.20)?;
    let input = replace_prob(input, ("i", &random_vowel_subst()), 0.20)?;
    let input = replace_prob(input, ("o", &random_vowel_subst()), 0.20)?;
    let input = replace_prob(input, ("E", "€"), 0.20)?;
    let input = replace_prob(input, ("u", "7"), 0.10)?;
    let input = replace_prob(input, ("y", "λ"), 0.10)?;
    let input = replace_prob(input, ("p", "b"), 0.10)?;
    let input = replace_prob(input, ("b", "p"), 0.10)?;
    let input = replace_prob(input, ("h", "j"), 0.10)?;
    let input = replace_prob(input, ("d", "n"), 0.10)?;
    let input = replace_prob(input, (",", ""), 0.40)?;
    let input = replace_prob(input, (".", ","), 0.40)?;
    let input = replace_prob(input, (" ", ""), 0.10)?;
    let input = replace_prob(input, ("r", "ww"), 0.10)?;
    let input = replace_prob(input, ("ae", "æ"), 0.70)?;
    let input = replace_prob(input, ("ea", "æ"), 0.70)?;
    let input = replace_prob(input, ("and", "&&"), 0.70)?;
    let input = replace_prob(
        input,
        (
            "l",
            match get_random(0.50) {
                true => "I",
                false => "i",
            },
        ),
        0.30,
    )?;

    Ok(input)
}

fn replace_prob(input: String, to_match: (&str, &str), prob: f64) -> Result<String> {
    let mut substituted = String::new();
    let mut split = input.split(to_match.0).peekable();
    while {
        substituted.push_str(split.next().unwrap());
        split.peek().is_some()
    } {
        if get_random(prob) {
            substituted.push_str(to_match.1);
        } else {
            substituted.push_str(to_match.0)
        }
    }
    Ok(substituted)
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
            pair.swap(0, 1);
        }
        // make letters randomly uppercase
        pair.iter_mut().for_each(|x| random_uppercase(x, 0.02));
    }
    word.iter().collect::<String>()
}

fn random_uppercase(input: &mut char, prob: f64) -> () {
    if get_random(prob) {
        input.make_ascii_uppercase();
    }
}

fn random_vowel_subst() -> String {
    String::from(
        vec!["a", "e", "i", "o", "u", "*", "^", ""][(rand::random::<f64>() / 0.125) as usize],
    )
}

fn get_random(prob: f64) -> bool {
    (rand::random::<f64>() / prob) as i32 == 0
}
