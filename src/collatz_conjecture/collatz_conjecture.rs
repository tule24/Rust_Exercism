pub fn collatz_conjecture() {}
pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {return None}
    let mut count = 0;
    while n != 1{
        if n%2 == 0{
            n /= 2;
        } else {
            n = n.checked_mul(3)?.checked_add(1)?;
        }
        count += 1;
    }
    Some(count)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Some(0), collatz(1));
    }
    #[test]
    fn test_16() {
        assert_eq!(Some(4), collatz(16));
    }
    #[test]
    fn test_12() {
        assert_eq!(Some(9), collatz(12));
    }
    #[test]
    fn test_1000000() {
        assert_eq!(Some(152), collatz(1_000_000));
    }
    #[test]
    fn test_0() {
        assert_eq!(None, collatz(0));
    }
    #[test]
    fn test_110243094271() {
        let val = 110243094271;
        assert_eq!(None, collatz(val));
    }
    #[test]
    fn test_max_div_3() {
        let max = u64::MAX / 3;
        assert_eq!(None, collatz(max));
    }
    #[test]
    fn test_max_minus_1() {
        let max = u64::MAX - 1;
        assert_eq!(None, collatz(max));
    }
}
