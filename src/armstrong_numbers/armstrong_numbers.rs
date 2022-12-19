
pub fn armstrong_numbers(num: u32)  -> bool{
    let num_string = num.to_string();
    let len = num_string.len() as u32;
    let res = num_string.chars().fold(0, |mut acc, char| {
        let temp:u32 = char.to_string().parse().unwrap();
        acc += temp.pow(len);
        acc
    });
    res == num
}

pub fn armstrong_numbers_2(num: u32)  -> bool{
    let digits = ((num as f64).log10() + 1.).floor() as u32;
    // Thuật toán đếm số chữ số là lấy căn 10 của số đó + thêm 1 rồi làm tròn xuống
    (0..digits)
        .map(|i| (num / 10u32.pow(i) % 10).pow(digits))
        .sum::<u32>() == num
    // ví dụ 153
    // l1: 153 / 1 % 10   = 3
    // l2: 153 / 10 % 10  = 5
    // l3: 153 / 100 % 10 = 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true(){
        assert!(armstrong_numbers_2(9))
    }

    #[test]
    fn test_true_2(){
        assert!(armstrong_numbers_2(153))
    }

    #[test]
    fn test_false(){
        assert_eq!(armstrong_numbers_2(154), false)
    }

    #[test]
    fn test_false_2(){
        assert_eq!(armstrong_numbers_2(10), false)
    }
}
