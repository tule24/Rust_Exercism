use std::collections::{HashMap, HashSet};

pub fn isogram() {}
pub fn check2(candidate: &str) -> bool {
    let mut map  = HashMap::new();
    for c in candidate.to_lowercase().chars(){
        if c.is_alphabetic() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
    }
    !map.into_iter().any(|(_, n)| n > 1)
}

pub fn check(candidate: &str) -> bool {
    let mut set  = HashSet::new();
    candidate.to_lowercase()
             .chars()
             .filter(|c| c.is_alphabetic())
             .all(|c| set.insert(c))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert!(check(""), "An empty string should be an isogram.")
    }
    #[test]
    fn only_lower_case_characters() {
        assert!(check("isogram"), "\"isogram\" should be an isogram.")
    }
    #[test]
    fn one_duplicated_character() {
        assert!(
            !check("eleven"),
            "\"eleven\" has more than one \'e\', therefore it is no isogram."
        )
    }
    #[test]
    fn longest_reported_english_isogram() {
        assert!(
            check("subdermatoglyphic"),
            "\"subdermatoglyphic\" should be an isogram."
        )
    }
    #[test]
    fn one_duplicated_character_mixed_case() {
        assert!(
            !check("Alphabet"),
            "\"Alphabet\" has more than one \'a\', therefore it is no isogram."
        )
    }
    #[test]
    fn hypothetical_isogramic_word_with_hyphen() {
        assert!(
            check("thumbscrew-japingly"),
            "\"thumbscrew-japingly\" should be an isogram."
        )
    }
    #[test]
    fn isogram_with_duplicated_hyphen() {
        assert!(
            check("six-year-old"),
            "\"six-year-old\" should be an isogram."
        )
    }
    #[test]
    fn made_up_name_that_is_an_isogram() {
        assert!(
            check("Emily Jung Schwartzkopf"),
            "\"Emily Jung Schwartzkopf\" should be an isogram."
        )
    }
    #[test]
    fn duplicated_character_in_the_middle() {
        assert!(
            !check("accentor"),
            "\"accentor\" has more than one \'c\', therefore it is no isogram."
        )
    }
}
