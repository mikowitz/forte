use crate::Set;

pub fn by(set: &Set, level: u32) -> Set {
    set.iter()
        .map(|pc| ((*pc).to_u32() + level).rem_euclid(12).into())
        .collect()
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
                #[doc = "# use forte::{Set, PitchClass::*};"]
                #[doc = "# let set: Set = vec![Cs, D, Ef];"]
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
    use crate::PitchClass::*;

    #[test]
    fn transposing_by_a_level() {
        let set: Set = vec![F, G, Af, B];
        let transposed: Set = by(&set, 8);
        let expected: Set = vec![Cs, Ef, E, G];

        assert_eq!(transposed, expected);
    }
}
