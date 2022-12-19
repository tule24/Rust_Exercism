pub fn proverb() {}

pub fn build_proverb(list: &[&str]) -> String {
    let mut vec = Vec::new();
    let len = list.len();

    if len == 0 {return format!("")}
    if len == 1 {return format!("And all for the want of a {}.", list[0])}
    (0..len - 1).for_each(
        |ele| vec.push(format!("For want of a {} the {} was lost.", list[ele], list[ele+1]))
    );
    vec.push(format!("And all for the want of a {}.", list[0]));
    vec.join("\n")
}

use std::iter::once;
pub fn build_proverb_2(list: &[&str]) -> String {
    match list.first() { // Lấy giá trị đầu tiên của list trả về Option<>
        None => String::new(),
        Some(word) => list.windows(2) // Tạo một iter với 2 phần tử 1 lượt 
            .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
            .chain(once(format!("And all for the want of a {}.", word))) //chain dùng để nối 2 iter thành 1, once tạo ra 1 iter đặc biệt, iter chỉ sử dụng 1 lần
            .collect(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_pieces() {
        let input = vec!["nail", "shoe"];
        let expected = vec![
            "For want of a nail the shoe was lost.",
            "And all for the want of a nail.",
        ]
        .join("\n");
        assert_eq!(build_proverb_2(&input), expected);
    }
    // Notice the change in the last line at three pieces.
    #[test]
    fn test_three_pieces() {
        let input = vec!["nail", "shoe", "horse"];
        let expected = vec![
            "For want of a nail the shoe was lost.",
            "For want of a shoe the horse was lost.",
            "And all for the want of a nail.",
        ]
        .join("\n");
        assert_eq!(build_proverb_2(&input), expected);
    }
    #[test]
    fn test_one_piece() {
        let input = vec!["nail"];
        let expected = String::from("And all for the want of a nail.");
        assert_eq!(build_proverb_2(&input), expected);
    }
    #[test]
    fn test_zero_pieces() {
        let input: Vec<&str> = vec![];
        let expected = String::new();
        assert_eq!(build_proverb_2(&input), expected);
    }
    #[test]
    fn test_full() {
        let input = vec![
            "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
        ];
        let expected = vec![
            "For want of a nail the shoe was lost.",
            "For want of a shoe the horse was lost.",
            "For want of a horse the rider was lost.",
            "For want of a rider the message was lost.",
            "For want of a message the battle was lost.",
            "For want of a battle the kingdom was lost.",
            "And all for the want of a nail.",
        ]
        .join("\n");
        assert_eq!(build_proverb_2(&input), expected);
    }
    #[test]
    fn test_three_pieces_modernized() {
        let input = vec!["pin", "gun", "soldier", "battle"];
        let expected = vec![
            "For want of a pin the gun was lost.",
            "For want of a gun the soldier was lost.",
            "For want of a soldier the battle was lost.",
            "And all for the want of a pin.",
        ]
        .join("\n");
        assert_eq!(build_proverb_2(&input), expected);
    }
}
