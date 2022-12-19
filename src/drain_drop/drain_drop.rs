use std::collections::BTreeMap;

pub fn drain_drop() {}

pub fn raindrops(n: u32) -> String {
    let mut res = String::new();
    let mut map = BTreeMap::new();
    map.insert(3, "Pling");
    map.insert(5, "Plang");
    map.insert(7, "Plong");

    map.iter().for_each(|(k, &v)| if n%k == 0 {res.push_str(v)});
    return if res.is_empty() {format!("{}",n)} else {res}
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("1", raindrops(1));
    }
    #[test]
    fn test_3() {
        assert_eq!("Pling", raindrops(3));
    }
    #[test]
    fn test_5() {
        assert_eq!("Plang", raindrops(5));
    }
    #[test]
    fn test_7() {
        assert_eq!("Plong", raindrops(7));
    }
    #[test]
    fn test_6() {
        assert_eq!("Pling", raindrops(6));
    }
    #[test]
    fn test_8() {
        assert_eq!("8", raindrops(8));
    }
    #[test]
    fn test_9() {
        assert_eq!("Pling", raindrops(9));
    }
    #[test]
    fn test_10() {
        assert_eq!("Plang", raindrops(10));
    }
    #[test]
    fn test_14() {
        assert_eq!("Plong", raindrops(14));
    }
    #[test]
    fn test_15() {
        assert_eq!("PlingPlang", raindrops(15));
    }
    #[test]
    fn test_21() {
        assert_eq!("PlingPlong", raindrops(21));
    }
    #[test]
    fn test_25() {
        assert_eq!("Plang", raindrops(25));
    }
    #[test]
    fn test_27() {
        assert_eq!("Pling", raindrops(27));
    }
    #[test]
    fn test_35() {
        assert_eq!("PlangPlong", raindrops(35));
    }
    #[test]
    fn test_49() {
        assert_eq!("Plong", raindrops(49));
    }
    #[test]
    fn test_52() {
        assert_eq!("52", raindrops(52));
    }
    #[test]
    fn test_105() {
        assert_eq!("PlingPlangPlong", raindrops(105));
    }
    #[test]
    fn test_3125() {
        assert_eq!("Plang", raindrops(3125));
    }
    #[test]
    fn test_12121() {
        assert_eq!("12121", raindrops(12_121));
    }
}
