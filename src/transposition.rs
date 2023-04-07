use crate::PitchClassSet;

pub fn by(set: &PitchClassSet, level: u32) -> PitchClassSet {
    let new_set = set
        .set()
        .iter()
        .map(|pc| ((*pc).to_u32() + level).rem_euclid(12).into())
        .collect();
    PitchClassSet::new(new_set)
}

#[macro_export]
#[doc(hidden)]
macro_rules! define_transpositions {
    ($($level:expr),*) => {
        $(
            paste! {
                #[doc = concat!("Transposes a pitch class [PitchClassSet] by ", stringify!($level), ".")]
                #[doc = "\n\n"]
                #[doc = "Equivalent to calling\n"]
                #[doc = "```\n"]
                #[doc = "# use forte::{PitchClassSet, set, PitchClass::*};"]
                #[doc = "# let set: PitchClassSet = set![Cs, D, Ef];"]
                #[doc = concat!("forte::transpose(&set, ", stringify!($level), ");\n")]
                #[doc = "```"]
                pub fn [<t $level>](set: &PitchClassSet) -> PitchClassSet {
                    transpose(set, $level)
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::{by, PitchClassSet};
    use crate::{set, PitchClass::*};

    #[test]
    fn transposing_by_a_level() {
        let set: PitchClassSet = set![F, G, Af, B];
        let transposed: PitchClassSet = by(&set, 8);
        let expected: PitchClassSet = set![Cs, Ef, E, G];

        assert_eq!(transposed, expected);
    }
}
