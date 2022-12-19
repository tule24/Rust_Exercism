pub fn palindrome() {}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    pub fn new(value: u64) -> Option<Palindrome> {
        let temp = value.to_string().chars().rev().collect::<String>().parse::<u64>().unwrap();
        return if temp == value {Some(Palindrome(value))} else {None}
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min >= max {return None};

    if max < 10{
        if Palindrome::new(min).is_some(){
            return Some((Palindrome(min), Palindrome(max)));
        }
    }

    if min%10 != 0 {return None};

    let res_min = (min+1)*(min+1);
    let res_max;
    let temp = max.to_string().len() as u32;
    if temp%2 == 0{
        let factor = 10_u32.pow(temp) - 11_u32.pow(temp/2 - 1)*9;
        res_max = max*(factor as u64);
    } else {
        let factor = (10_u32.pow(temp) - 7) as u64;
        res_max = factor*(factor-80);
    } 
    Some((Palindrome(res_min), Palindrome(res_max)))
}
#[cfg(test)]
mod tests {
    use super::*;
    fn process_smallest_case((from, to): (u64, u64), expected: Option<u64>) {
        let min = palindrome_products(from, to).map(|(min, _)| min);
        assert_eq!(min.map(|newtype| newtype.into_inner()), expected);
    }
    fn process_largest_case((from, to): (u64, u64), expected: Option<u64>) {
        let max = palindrome_products(from, to).map(|(_, max)| max);
        assert_eq!(max.map(|newtype| newtype.into_inner()), expected);
    }
    #[test]
    fn test_palindrome_new_return_some() {
        for v in [1, 11, 121, 12321, 1234321, 123454321, 543212345] {
            assert_eq!(Palindrome::new(v).expect("is a palindrome").into_inner(), v);
        }
    }
    #[test]
    fn test_palindrome_new_return_none() {
        for v in [12, 2322, 23443, 1233211, 8932343] {
            assert_eq!(Palindrome::new(v), None);
        }
    }
    #[test]
    fn test_finds_the_smallest_palindrome_from_single_digit_factors() {
        process_smallest_case((1, 9), Some(1));
    }
    #[test]
    fn test_finds_the_largest_palindrome_from_single_digit_factors() {
        process_largest_case((1, 9), Some(9));
    }
    #[test]
    fn test_find_the_smallest_palindrome_from_double_digit_factors() {
        process_smallest_case((10, 99), Some(121));
    }
    #[test]
    fn test_find_the_largest_palindrome_from_double_digit_factors() {
        process_largest_case((10, 99), Some(9009));
    }
    #[test]
    fn test_find_smallest_palindrome_from_triple_digit_factors() {
        process_smallest_case((100, 999), Some(10201));
    }
    #[test]
    fn test_find_the_largest_palindrome_from_triple_digit_factors() {
        process_largest_case((100, 999), Some(906609));
    }
    #[test]
    fn test_find_smallest_palindrome_from_four_digit_factors() {
        process_smallest_case((1000, 9999), Some(1002001));
    }
    #[test]
    fn test_find_the_largest_palindrome_from_four_digit_factors() {
        process_largest_case((1000, 9999), Some(99000099));
    }
    #[test]
    fn test_empty_result_for_smallest_if_no_palindrome_in_the_range() {
        process_smallest_case((1002, 1003), None);
    }
    #[test]
    fn test_empty_result_for_largest_if_no_palindrome_in_the_range() {
        process_largest_case((15, 15), None);
    }
    #[test]
    fn test_error_result_for_smallest_if_min_is_more_than_max() {
        process_smallest_case((10000, 1), None);
    }
    #[test]
    fn test_error_result_for_largest_if_min_is_more_than_max() {
        process_largest_case((2, 1), None);
    }
}
