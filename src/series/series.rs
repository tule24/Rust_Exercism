pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string();digits.len() + 1]
    }
    let res = digits.chars().map(|f| f.to_string()).collect::<Vec<String>>();
    res.windows(len).fold(Vec::<String>::new(), |mut acc, x| {
        acc.push(x.join(""));
        acc
    })
}

pub fn series_2(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }
    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|c| c.into_iter().collect::<String>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_zero_length() {
        let expected = vec!["".to_string(); 6];
        assert_eq!(series("92017", 0), expected);
    }
    #[test]
    fn test_with_length_2() {
        let expected = vec![
            "92".to_string(),
            "20".to_string(),
            "01".to_string(),
            "17".to_string(),
        ];
        assert_eq!(series("92017", 2), expected);
    }
    #[test]
    fn test_with_numbers_length() {
        let expected = vec!["92017".to_string()];
        assert_eq!(series("92017", 5), expected);
    }
    #[test]
    fn test_too_long() {
        let expected: Vec<String> = vec![];
        assert_eq!(series("92017", 6), expected);
    }
    #[test]
    fn test_way_too_long() {
        let expected: Vec<String> = vec![];
        assert_eq!(series("92017", 42), expected);
    }
}
