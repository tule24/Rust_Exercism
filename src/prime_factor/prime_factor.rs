pub fn prime_factor() {}

pub fn factors(mut n: u64) -> Vec<u64> {
    let mut res:Vec<u64> = Vec::new();
    let mut prime = 2u64;
    while n > 1 {
        if n % prime == 0 {
            n /= prime;
            res.push(prime);
            continue;
        }
        prime = if prime > 2 {prime + 2} else {prime + 1};
    }
    res
}

pub fn factors_2(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    match (2..n).find(|x| n%x == 0) {
        Some(x) => {
            result.push(x);
            result.append(&mut factors_2(n/x));
        },
        None => {}
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_factors() {
        assert_eq!(factors(1), vec![]);
    }
    #[test]
    fn test_prime_number() {
        assert_eq!(factors(2), vec![2]);
    }
    #[test]
    fn test_square_of_a_prime() {
        assert_eq!(factors(9), vec![3, 3]);
    }
    #[test]
    fn test_cube_of_a_prime() {
        assert_eq!(factors(8), vec![2, 2, 2]);
    }
    #[test]
    fn test_product_of_primes_and_non_primes() {
        assert_eq!(factors(12), vec![2, 2, 3]);
    }
    #[test]
    fn test_product_of_primes() {
        assert_eq!(factors(901_255), vec![5, 17, 23, 461]);
    }
    #[test]
    fn test_factors_include_large_prime() {
        assert_eq!(factors(93_819_012_551), vec![11, 9539, 894_119]);
    }
}
