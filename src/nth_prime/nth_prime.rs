pub fn nth_prime() {}

pub fn nth(n: u32) -> u32 {
    if n == 0 {return 2;}
    let mut i = 3;
    let mut count = 1;
    while count != n{
        i += 2;
        if check_prime(i) {
            count += 1;
        }
    }
    i
}

pub fn check_prime(n: u32) -> bool {
    if n < 2 {return false;}
    if (n == 2 || n == 3) {return true;}
    if (n%2 == 0 || n%3 == 0) {return false;}
    let max = (n as f32).sqrt() as u32;
    let mut i = 5;
    while i <= max{
        if n%i == 0 || n%(i+2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_prime() {
        assert_eq!(nth(0), 2);
    }
    #[test]
    fn test_second_prime() {
        assert_eq!(nth(1), 3);
    }
    #[test]
    fn test_sixth_prime() {
        assert_eq!(nth(5), 13);
    }
    #[test]
    fn test_big_prime() {
        assert_eq!(nth(10_000), 104_743);
    }
}
