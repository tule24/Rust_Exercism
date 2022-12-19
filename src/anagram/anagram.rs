pub fn anagram() {}
use std::collections::HashSet;
/* EXAMPLE HASHSET
let mut a: HashSet<i32> = vec![1i32,2,3].into_iter().collect();
let mut b: HashSet<i32> = vec![4i32,5,2].into_iter().collect();

// union: get all the unique elements in both sets.
println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());
// difference: get all the elements that are in the first set but not the second.
println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());
// intersection: get all the elements that are only in both sets.
println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());
// symmetric_difference: get all the elements that are in one set or the other, but not both.
println!("Symetric_difference: {:?}", a.symmetric_difference(&b).collect::<Vec<&i32>>());
*/

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let old_word = word.to_lowercase();
    let mut new_word = old_word.chars().collect::<Vec<char>>();
    new_word.sort_unstable();

    possible_anagrams
        .iter()
        .fold(HashSet::new(), |mut acc, val| {
            let old_val = val.to_lowercase();
            let mut new_val = old_val.chars().collect::<Vec<char>>();
            new_val.sort_unstable();
            if new_word == new_val && old_word != old_val{
                acc.insert(*val);
            };
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
        let result = anagrams_for(word, inputs);
        let expected: HashSet<&str> = expected.iter().cloned().collect();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_no_matches() {
        let word = "diaper";
        let inputs = ["hello", "world", "zombies", "pants"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_detect_simple_anagram() {
        let word = "ant";
        let inputs = ["tan", "stand", "at"];
        let outputs = vec!["tan"];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_does_not_confuse_different_duplicates() {
        let word = "galea";
        let inputs = ["eagle"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_eliminate_anagram_subsets() {
        let word = "good";
        let inputs = ["dog", "goody"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_detect_anagram() {
        let word = "listen";
        let inputs = ["enlists", "google", "inlets", "banana"];
        let outputs = vec!["inlets"];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_multiple_anagrams() {
        let word = "allergy";
        let inputs = [
            "gallery",
            "ballerina",
            "regally",
            "clergy",
            "largely",
            "leading",
        ];
        let outputs = vec!["gallery", "regally", "largely"];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_case_insensitive_anagrams() {
        let word = "Orchestra";
        let inputs = ["cashregister", "Carthorse", "radishes"];
        let outputs = vec!["Carthorse"];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_unicode_anagrams() {
        let word = "ΑΒΓ";
        // These words don't make sense, they're just greek letters cobbled together.
        let inputs = ["ΒΓΑ", "ΒΓΔ", "γβα"];
        let outputs = vec!["ΒΓΑ", "γβα"];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_misleading_unicode_anagrams() {
        // Despite what a human might think these words contain different letters, the input uses Greek
        // A and B while the list of potential anagrams uses Latin A and B.
        let word = "ΑΒΓ";
        let inputs = ["ABΓ"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_does_not_detect_a_word_as_its_own_anagram() {
        let word = "banana";
        let inputs = ["banana"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_does_not_detect_a_differently_cased_word_as_its_own_anagram() {
        let word = "banana";
        let inputs = ["bAnana"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_does_not_detect_a_differently_cased_unicode_word_as_its_own_anagram() {
        let word = "ΑΒΓ";
        let inputs = ["ΑΒγ"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_same_bytes_different_chars() {
        let word = "a⬂"; // 61 E2 AC 82
        let inputs = ["€a"]; // E2 82 AC 61
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_different_words_but_same_ascii_sum() {
        let word = "bc";
        let inputs = ["ad"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
}
