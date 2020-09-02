use std::collections::{HashMap, HashSet};

fn get_counts(word: &str) -> HashMap<char, u8> {
    let mut letters = <HashMap<char, u8>>::new();
    for letter in word.chars() {
        let count = letters.entry(letter).or_insert(0);
        *count += 1;
    }
    letters
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = &word.to_lowercase();
    let counts = get_counts(word_lower);

    possible_anagrams
        .iter()
        .filter_map(|&candidate| {
            let candidate_lower = &candidate.to_lowercase();
            if candidate_lower != word_lower && get_counts(candidate_lower) == counts {
                Some(candidate)
            } else {
                None
            }
        })
        .collect()
}
