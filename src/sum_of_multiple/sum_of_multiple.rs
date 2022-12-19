use std::collections::HashMap;

pub fn sum_of_multiple() {}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
   let mut res = HashMap::new();
   factors
    .iter()
    .for_each(|num|{
        let mut multiple = *num;
        while limit > multiple {
            res.insert(multiple, 0);
            multiple += num; 
            if multiple == 0 {break;}
        }
   });
   res.into_keys().sum()
}

pub fn sum_of_multiples_3(limit: u32, factors: &[u32]) -> u32 {
    let mut res = Vec::<u32>::new();
    factors
     .iter()
     .for_each(|num|{
         if num != &0 {
            let mut multiple = *num;
            while limit > multiple {
                res.push(multiple);
                multiple += num; 
            }
         }
    });
    res.sort();
    res.dedup();
    res.iter().sum()
 }

 pub fn sum_of_multiples_2(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|num| factors.iter().any(|&f| f != 0 && num % f == 0))
        .sum()
 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_multiples_within_limit() {
        assert_eq!(0, sum_of_multiples_2(1, &[3, 5]))
    }
    #[test]
    fn one_factor_has_multiples_within_limit() {
        assert_eq!(3, sum_of_multiples_2(4, &[3, 5]))
    }
    #[test]
    fn more_than_one_multiple_within_limit() {
        assert_eq!(9, sum_of_multiples_2(7, &[3]))
    }
    #[test]
    fn more_than_one_factor_with_multiples_within_limit() {
        assert_eq!(23, sum_of_multiples_2(10, &[3, 5]))
    }
    #[test]
    fn each_multiple_is_only_counted_once() {
        assert_eq!(2318, sum_of_multiples_2(100, &[3, 5]))
    }
    #[test]
    fn a_much_larger_limit() {
        assert_eq!(233_168, sum_of_multiples_2(1000, &[3, 5]))
    }
    #[test]
    fn three_factors() {
        assert_eq!(51, sum_of_multiples_2(20, &[7, 13, 17]))
    }
    #[test]
    fn factors_not_relatively_prime() {
        assert_eq!(30, sum_of_multiples_2(15, &[4, 6]))
    }
    #[test]
    fn some_pairs_of_factors_relatively_prime_and_some_not() {
        assert_eq!(4419, sum_of_multiples_2(150, &[5, 6, 8]))
    }
    #[test]
    fn one_factor_is_a_multiple_of_another() {
        assert_eq!(275, sum_of_multiples_2(51, &[5, 25]))
    }
    #[test]
    fn much_larger_factors() {
        assert_eq!(2_203_160, sum_of_multiples_2(10_000, &[43, 47]))
    }
    #[test]
    fn all_numbers_are_multiples_of_1() {
        assert_eq!(4950, sum_of_multiples_2(100, &[1]))
    }
    #[test]
    fn no_factors_means_an_empty_sum() {
        assert_eq!(0, sum_of_multiples_2(10_000, &[]))
    }
    #[test]
    fn the_only_multiple_of_0_is_0() {
        assert_eq!(0, sum_of_multiples_2(1, &[0]))
    }
    #[test]
    fn the_factor_0_does_not_affect_the_sum_of_multiples_of_other_factors() {
        assert_eq!(3, sum_of_multiples_2(4, &[3, 0]))
    }
    #[test]
    fn solutions_using_include_exclude_must_extend_to_cardinality_greater_than_3() {
        assert_eq!(39_614_537, sum_of_multiples_2(10_000, &[2, 3, 5, 7, 11]))
    }
}
