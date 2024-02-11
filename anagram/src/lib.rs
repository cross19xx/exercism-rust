use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut output: HashSet<&str> = HashSet::from_iter([]);

    let lowercased_word = word.to_lowercase();
    let word_vector = Vec::from_iter(lowercased_word.chars());
    let word_byte_count: u16 = lowercased_word
        .as_bytes()
        .iter()
        .map(|&byte| byte as u16)
        .sum();

    for anagram in possible_anagrams {
        let lowercased_anagram = anagram.to_lowercase();
        let anagram_vector = Vec::from_iter(lowercased_anagram.chars());
        let anagram_byte_count: u16 = lowercased_anagram
            .as_bytes()
            .iter()
            .map(|&byte| byte as u16)
            .sum();

        if anagram_vector.iter().all(|item| word_vector.contains(item))
            && lowercased_anagram != lowercased_word
            && word_byte_count == anagram_byte_count
        {
            output.insert(anagram);
        }
    }

    output
}
