pub type Set = Vec<u32>;

mod inversion;
mod normal_form;
mod prime_form;
mod transposition;
mod utils;

use paste::paste;

pub fn to_normal_form(set: &Set) -> Set {
    normal_form::from(set)
}

pub fn to_prime_form(set: &Set) -> Set {
    prime_form::from(set)
}

pub fn transpose(set: &Set, level: u32) -> Set {
    transposition::by(set, level)
}

pub fn invert(set: &Set, level: u32) -> Set {
    inversion::by(set, level)
}

pub fn invert_by_pair(set: &Set, inversion_pair: (u32, u32)) -> Set {
    inversion::by_pair(set, inversion_pair)
}

define_transpositions!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
define_inversions!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);

#[cfg(test)]
mod tests {
    use crate::{i5, invert, invert_by_pair, t3, to_prime_form, transpose, Set};

    #[test]
    fn transposition() {
        let set: Set = vec![7, 8, 10, 11];

        assert_eq!(transpose(&set, 3), vec![10, 11, 1, 2]);
    }

    #[test]
    fn generated_transposition_levels() {
        let set: Set = vec![7, 8, 10, 11];

        assert_eq!(t3(&set), vec![10, 11, 1, 2]);
    }

    #[test]
    fn inversion() {
        let set: Set = vec![1, 3, 4, 7];

        assert_eq!(invert(&set, 5), vec![10, 1, 2, 4]);
    }

    #[test]
    fn inversion_by_pair() {
        let set: Set = vec![7, 8, 11];

        assert_eq!(invert_by_pair(&set, (7, 11)), vec![7, 10, 11]);
    }

    #[test]
    fn generated_inversion_levels() {
        let set: Set = vec![1, 3, 4, 7];

        assert_eq!(i5(&set), vec![10, 1, 2, 4]);
    }

    #[test]
    fn to_normal_form() {
        let set: Set = vec![10, 2, 5, 6];

        assert_eq!(to_prime_form(&set), vec![0, 1, 4, 8]);
    }
}
