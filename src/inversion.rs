use crate::{PitchClass, PitchClassSet};

pub fn by(set: &PitchClassSet, level: u32) -> PitchClassSet {
    let new_set: Vec<PitchClass> = set
        .set()
        .iter()
        .map(|pc| ((level as i32 - (*pc).to_u32() as i32).rem_euclid(12) as u32).into())
        .rev()
        .collect();
    PitchClassSet::new(new_set)
}

pub fn by_pair(set: &PitchClassSet, inversion_pair: (PitchClass, PitchClass)) -> PitchClassSet {
    let (a, b) = inversion_pair;
    let level = (a.to_u32() + b.to_u32()).rem_euclid(12);
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
                #[doc = concat!("Inverts a [PitchClassSet] by ", stringify!($level), ".")]
                #[doc = "\n\n"]
                #[doc = "Equivalent to calling\n"]
                #[doc = "```\n"]
                #[doc = "# use forte::{PitchClassSet, set, PitchClass::*};"]
                #[doc = "# let set: PitchClassSet = set![Cs, D, Ef];"]
                #[doc = concat!("forte::invert(&set, ", stringify!($level), ");\n")]
                #[doc = "```"]
                pub fn [<i $level>](set: &PitchClassSet) -> PitchClassSet {
                    invert(set, $level)
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::{by, by_pair, PitchClass::*, PitchClassSet};
    use crate::set;

    #[test]
    fn inverting_around_a_level() {
        let set: PitchClassSet = set![Cs, Ef, F, G];
        let inverted: PitchClassSet = by(&set, 5);
        assert_eq!(inverted, set![Bf, C, D, E]);
    }

    #[test]
    fn i0_is_not_an_identity() {
        let set: PitchClassSet = set![G, Af, B];
        let inverted = by(&set, 0);
        assert_eq!(inverted, set![Cs, E, F]);
    }

    #[test]
    fn inverting_by_an_inversion_pair() {
        let set: PitchClassSet = set![G, Af, B];
        let inverted = by_pair(&set, (G, B));
        assert_eq!(inverted, set![G, Bf, B]);
    }
}
