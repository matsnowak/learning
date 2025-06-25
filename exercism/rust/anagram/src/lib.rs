use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::<&'a str>::new();
    for w in possible_anagrams {
        if is_anagrams(word, w) {
            result.insert(w);
        }
    }
    result
}

pub fn is_anagrams(word_a: &str, word_b: &str) -> bool {
    if word_a.to_lowercase() == word_b.to_lowercase() {
        return false;
    }
    let mut letters_map: HashMap<char, i8> = HashMap::new();
    for a in word_a.to_lowercase().chars() {
        letters_map
            .entry(a)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    for b in word_b.to_lowercase().chars() {
        letters_map
            .entry(b)
            .and_modify(|counter| *counter -= 1)
            .or_insert(-1);
    }

    letters_map.values().filter(|x| 0 != **x).count() == 0
}

#[cfg(test)]
mod test {
    use crate::is_anagrams;

    #[test]
    fn is_anagram_true() {
        assert!(is_anagrams("dog", "god"));
    }

    #[test]
    fn is_anagram_false() {
        assert!(!is_anagrams("dog", "goda"));
    }
}
