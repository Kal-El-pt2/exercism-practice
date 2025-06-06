use std::collections::{HashMap, HashSet};

pub fn char_count_map(word: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in word.chars().flat_map(|c| c.to_lowercase()) {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let word_lower = word.chars().flat_map(|c| c.to_lowercase()).collect::<String>();
    let word_map = char_count_map(word);

    possible_anagrams
        .iter()
        .copied()
        .filter(|&candidate| {
            let candidate_lower = candidate.chars().flat_map(|c| c.to_lowercase()).collect::<String>();
            candidate_lower != word_lower && char_count_map(candidate) == word_map
        })
        .collect()
}
