use crate::Set;

pub fn by(set: &Set, level: u32) -> Set {
    set.iter().map(|pc| (pc + level).rem_euclid(12)).collect()
}

#[macro_export]
#[doc(hidden)]
macro_rules! define_transpositions {
    ($($level:expr),*) => {
        $(
            paste! {
                #[doc = concat!("Transposes a pitch class [set][Set] by ", stringify!($level), ".")]
                #[doc = "\n\n"]
                #[doc = "Equivalent to calling\n"]
                #[doc = "```\n"]
                #[doc = "# let set = vec![1,2,3];"]
                #[doc = concat!("forte::transpose(&set, ", stringify!($level), ");\n")]
                #[doc = "```"]
                pub fn [<t $level>](set: &Set) -> Set {
                    transpose(set, $level)
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::{by, Set};

    #[test]
    fn transposing_by_a_level() {
        let set: Set = vec![5, 7, 8, 11];
        let transposed: Set = by(&set, 8);
        let expected: Set = vec![1, 3, 4, 7];

        assert_eq!(transposed, expected);
    }
}
