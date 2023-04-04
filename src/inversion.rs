use crate::Set;

pub fn by(set: &Set, level: u32) -> Set {
    set.iter()
        .map(|pc| (level as i32 - *pc as i32).rem_euclid(12) as u32)
        .rev()
        .collect()
}

pub fn by_pair(set: &Set, inversion_pair: (u32, u32)) -> Set {
    let (a, b) = inversion_pair;
    let level = (a + b).rem_euclid(12);
    by(set, level)
}

#[macro_export]
#[doc(hidden)]
macro_rules! define_inversions {
    (
        $($level:expr) , *
    ) => {
        $(
            paste! {
                #[doc = concat!("Inverts a pitch class [set][Set] by ", stringify!($level), ".")]
                #[doc = "\n\n"]
                #[doc = "Equivalent to calling\n"]
                #[doc = "```\n"]
                #[doc = "# let set = vec![1,2,3];"]
                #[doc = concat!("forte::invert(&set, ", stringify!($level), ");\n")]
                #[doc = "```"]
                pub fn [<i $level>](set: &Set) -> Set {
                    invert(set, $level)
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::{by, by_pair, Set};

    #[test]
    fn inverting_around_a_level() {
        let set: Set = vec![1, 3, 4, 7];
        let inverted: Set = by(&set, 5);
        assert_eq!(inverted, vec![10, 1, 2, 4]);
    }

    #[test]
    fn i0_is_not_an_identity() {
        let set: Set = vec![7, 8, 11];
        let inverted = by(&set, 0);
        assert_eq!(inverted, vec![1, 4, 5]);
    }

    #[test]
    fn inverting_by_an_inversion_pair() {
        let set: Set = vec![7, 8, 11];
        let inverted = by_pair(&set, (7, 11));
        assert_eq!(inverted, vec![7, 10, 11]);
    }
}
