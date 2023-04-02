pub type Set = Vec<u32>;

mod normal_form;
mod transposition;

use paste::paste;

pub fn to_normal_form(set: &Set) -> Set {
    normal_form::from(set)
}

pub fn transpose(set: &Set, level: u32) -> Set {
    transposition::by(set, level)
}

define_transpositions!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);

#[cfg(test)]
mod tests {
    use crate::{t1, Set};

    #[test]
    fn generated_transposition_levels() {
        let set: Set = vec![0, 1, 2];

        assert_eq!(t1(&set), vec![1, 2, 3]);
    }
}
