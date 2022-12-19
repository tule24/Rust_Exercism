pub fn acronym() {}
pub fn abbreviate_2(phrase: &str) -> String {
    phrase.split([' ', '-', '_', ':'])
          .filter(|f| f.len() > 0)
          .map(|f| {
            if f.chars().all(|x| x.is_uppercase()) {f.get(0..1).unwrap().to_uppercase()}
            else if f.chars().all(|x| x.is_lowercase()) {f.get(0..1).unwrap().to_uppercase()}
            else {f.chars().filter(|x| x.is_uppercase()).collect::<String>()}
          })
          .collect()
}
/*
1. Split dùng để cắt phrase thành các từ riêng biệt
2. Flat_map dùng để trải mảng,các iter trong đó sẽ được trải ra thành 1 iter duy nhất
3. Tạo iter, lấy ký tự đầu tiên của mỗi từ thông qua take() => return 1 iter với số lượng phần tử được nhập trong take
4. Dùng chain để nối iter take kia với iter mới được tạo lại từ word
5. skip_while dùng để bỏ qua các phần từ uppercase tiếp theo cho tới khi thấy được phần tử lowercase tiếp
6. filter lặp qua các từ còn lại lấy từ uppercase ra
 */
pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_ascii_whitespace() || c == '_' || c == '-')
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_ascii_uppercase())
                    .filter(|c| c.is_ascii_uppercase()),
            )
        })
        .collect::<String>()
        .to_ascii_uppercase()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(abbreviate(""), "");
    }
    #[test]
    fn basic() {
        assert_eq!(abbreviate("Portable Network Graphics"), "PNG");
    }
    #[test]
    fn lowercase_words() {
        assert_eq!(abbreviate("Ruby on Rails"), "ROR");
    }
    #[test]
    fn camelcase() {
        assert_eq!(abbreviate("HyperText Markup Language"), "HTML");
    }
    #[test]
    fn punctuation() {
        assert_eq!(abbreviate("First In, First Out"), "FIFO");
    }
    #[test]
    fn all_caps_word() {
        assert_eq!(
            abbreviate("GNU Image Manipulation Program"),
            "GIMP"
        );
    }
    #[test]
    fn all_caps_word_with_punctuation() {
        assert_eq!(abbreviate("PHP: Hypertext Preprocessor"), "PHP");
    }
    #[test]
    fn punctuation_without_whitespace() {
        assert_eq!(
            abbreviate("Complementary metal-oxide semiconductor"),
            "CMOS"
        );
    }
    #[test]
    fn very_long_abbreviation() {
        assert_eq!(
            abbreviate(
                "Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me"
            ),
            "ROTFLSHTMDCOALM"
        );
    }
    #[test]
    fn consecutive_delimiters() {
        assert_eq!(
            abbreviate("Something - I made up from thin air"),
            "SIMUFTA"
        );
    }
    #[test]
    fn apostrophes() {
        assert_eq!(abbreviate("Halley's Comet"), "HC");
    }
    #[test]
    fn underscore_emphasis() {
        assert_eq!(abbreviate("The Road _Not_ Taken"), "TRNT");
    }
}
