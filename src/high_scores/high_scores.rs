pub fn high_scores() {}
#[derive(Debug)]
pub struct HighScores(Vec<u32>);

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores(Vec::from(scores))
    }

    pub fn scores(&self) -> &[u32] {
        &self.0
    }

    pub fn latest(&self) -> Option<u32> {
        if self.0.len() != 0{
            return Some(self.0[self.0.len() - 1])
        }
        None
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut max = 0;
        self.0.iter().for_each(|ele| if *ele > max {max = *ele});
        // self.0.iter().max()
        return if max != 0 {Some(max)} else {None}
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut temp = self.0.clone();
        temp.sort();
        temp.reverse();
        if temp.len() < 3 {return temp}
        temp.truncate(3);
        temp
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_of_scores() {
        let expected = [30, 50, 20, 70];
        let high_scores = HighScores::new(&expected);
        assert_eq!(high_scores.scores(), &expected);
    }
    #[test]
    fn test_latest_score() {
        let high_scores = HighScores::new(&[100, 0, 90, 30]);
        assert_eq!(high_scores.latest(), Some(30));
    }
    #[test]
    fn test_latest_score_empty() {
        let high_scores = HighScores::new(&[]);
        assert_eq!(high_scores.latest(), None);
    }
    #[test]
    fn test_personal_best() {
        let high_scores = HighScores::new(&[40, 100, 70]);
        assert_eq!(high_scores.personal_best(), Some(100));
    }
    #[test]
    fn test_personal_best_empty() {
        let high_scores = HighScores::new(&[]);
        assert_eq!(high_scores.personal_best(), None);
    }
    #[test]
    fn test_personal_top_three() {
        let high_scores = HighScores::new(&[10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70]);
        assert_eq!(high_scores.personal_top_three(), vec![100, 90, 70]);
    }
    #[test]
    fn test_personal_top_three_highest_to_lowest() {
        let high_scores = HighScores::new(&[20, 10, 30]);
        assert_eq!(high_scores.personal_top_three(), vec![30, 20, 10]);
    }
    #[test]
    fn test_personal_top_three_with_tie() {
        let high_scores = HighScores::new(&[40, 20, 40, 30]);
        assert_eq!(high_scores.personal_top_three(), vec![40, 40, 30]);
    }
    #[test]
    fn test_personal_top_three_with_less_than_three_scores() {
        let high_scores = HighScores::new(&[30, 70]);
        assert_eq!(high_scores.personal_top_three(), vec![70, 30]);
    }
    #[test]
    fn test_personal_top_three_only_one_score() {
        let high_scores = HighScores::new(&[40]);
        assert_eq!(high_scores.personal_top_three(), vec![40]);
    }
    #[test]
    fn test_personal_top_three_empty() {
        let high_scores = HighScores::new(&[]);
        assert!(high_scores.personal_top_three().is_empty());
    }
}
