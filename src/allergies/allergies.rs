pub fn allergies() {}
// #[derive(Debug, Clone)]
// pub struct Allergies {
//     allergen: Vec<Allergen>,
// }

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub enum Allergen {
//     Eggs,
//     Peanuts,
//     Shellfish,
//     Strawberries,
//     Tomatoes,
//     Chocolate,
//     Pollen,
//     Cats,
// }

// impl Allergies {
//     pub fn new(score: u32) -> Self {
//         if score < 1 {
//             return Self { allergen: vec![] };
//         }
//         let mut new_score = score % 256;
//         let mut allergen = Vec::<Allergen>::new();
//         while new_score > 0 {
//             match new_score {
//                 s if s >= 128 => {
//                     new_score -= 128;
//                     allergen.push(Allergen::Cats);
//                 }
//                 s if s >= 64 => {
//                     new_score -= 64;
//                     allergen.push(Allergen::Pollen);
//                 }
//                 s if s >= 32 => {
//                     new_score -= 32;
//                     allergen.push(Allergen::Chocolate);
//                 }
//                 s if s >= 16 => {
//                     new_score -= 16;
//                     allergen.push(Allergen::Tomatoes);
//                 }
//                 s if s >= 8 => {
//                     new_score -= 8;
//                     allergen.push(Allergen::Strawberries);
//                 }
//                 s if s >= 4 => {
//                     new_score -= 4;
//                     allergen.push(Allergen::Shellfish);
//                 }
//                 s if s >= 2 => {
//                     new_score -= 2;
//                     allergen.push(Allergen::Peanuts);
//                 }
//                 _ => {
//                     allergen.push(Allergen::Eggs);
//                     break;
//                 }
//             }
//         }
//         allergen.reverse();
//         Self { allergen }
//     }

//     pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
//         self.allergen.contains(allergen)
//     }

//     pub fn allergies(&self) -> Vec<Allergen> {
//         self.clone().allergen
//     }
// }

pub struct Allergies { score: u8 }
#[derive(Debug, PartialEq, Copy, Clone)]

pub enum Allergen {
    Eggs = 1, // 01
    Peanuts = 2, // 10
    Shellfish = 4, // 100
    Strawberries = 8, // 1000
    Tomatoes = 16, // 100000
    Chocolate = 32, // ....
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score: score as u8 }
    }
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & *allergen as u8 != 0 // bitwise AND
        // != 0 khi self.score > allergen
    }
    pub fn allergies(&self) -> Vec<Allergen> {
        let all_of_them = vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats
        ];
        all_of_them.into_iter().filter(|al| self.is_allergic_to(al)).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn compare_allergy_vectors(expected: &[Allergen], actual: &[Allergen]) {
        for element in expected {
            if !actual.contains(element) {
                panic!(
                    "Allergen missing\n  {:?} should be in {:?}",
                    element, actual
                );
            }
        }
        if actual.len() != expected.len() {
            panic!(
                "Allergy vectors are of different lengths\n  expected {:?}\n  got {:?}",
                expected, actual
            );
        }
    }
    #[test]
    fn is_not_allergic_to_anything() {
        let allergies = Allergies::new(0);
        assert!(!allergies.is_allergic_to(&Allergen::Peanuts));
        assert!(!allergies.is_allergic_to(&Allergen::Cats));
        assert!(!allergies.is_allergic_to(&Allergen::Strawberries));
    }
    #[test]
    fn is_allergic_to_eggs() {
        assert!(Allergies::new(1).is_allergic_to(&Allergen::Eggs));
    }
    #[test]
    fn is_allergic_to_eggs_and_shellfish_but_not_strawberries() {
        let allergies = Allergies::new(5);
        assert!(allergies.is_allergic_to(&Allergen::Eggs));
        assert!(allergies.is_allergic_to(&Allergen::Shellfish));
        assert!(!allergies.is_allergic_to(&Allergen::Strawberries));
    }
    #[test]
    fn no_allergies_at_all() {
        let expected = &[];
        let allergies = Allergies::new(0).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
    #[test]
    fn allergic_to_just_eggs() {
        let expected = &[Allergen::Eggs];
        let allergies = Allergies::new(1).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
    #[test]
    fn allergic_to_just_peanuts() {
        let expected = &[Allergen::Peanuts];
        let allergies = Allergies::new(2).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
    #[test]
    fn allergic_to_just_strawberries() {
        let expected = &[Allergen::Strawberries];
        let allergies = Allergies::new(8).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
    #[test]
    fn allergic_to_eggs_and_peanuts() {
        let expected = &[Allergen::Eggs, Allergen::Peanuts];
        let allergies = Allergies::new(3).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
    #[test]
    fn allergic_to_eggs_and_shellfish() {
        let expected = &[Allergen::Eggs, Allergen::Shellfish];
        let allergies = Allergies::new(5).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
    #[test]
    fn allergic_to_many_things() {
        let expected = &[
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];
        let allergies = Allergies::new(248).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
    #[test]
    fn allergic_to_everything() {
        let expected = &[
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];
        let allergies = Allergies::new(255).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
    #[test]
    fn scores_over_255_do_not_trigger_false_positives() {
        let expected = &[
            Allergen::Eggs,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];
        let allergies = Allergies::new(509).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
}
