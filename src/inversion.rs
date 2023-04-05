use crate::{PitchClass, Set};

pub fn by(set: &Set, level: u32) -> Set {
    set.iter()
        .map(|pc| ((level as i32 - (*pc).to_u32() as i32).rem_euclid(12) as u32).into())
        .rev()
        .collect()
}

pub fn by_pair(set: &Set, inversion_pair: (PitchClass, PitchClass)) -> Set {
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
                #[doc = concat!("Inverts a pitch class [set][Set] by ", stringify!($level), ".")]
                #[doc = "\n\n"]
                #[doc = "Equivalent to calling\n"]
                #[doc = "```\n"]
                #[doc = "# use forte::{Set, PitchClass::*};"]
                #[doc = "# let set: Set = vec![Cs, D, Ef];"]
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
    use super::{by, by_pair, PitchClass::*, Set};

    #[test]
    fn inverting_around_a_level() {
        let set: Set = vec![Cs, Ef, F, G];
        let inverted: Set = by(&set, 5);
        assert_eq!(inverted, vec![Bf, C, D, E]);
    }

    #[test]
    fn i0_is_not_an_identity() {
        let set: Set = vec![G, Af, B];
        let inverted = by(&set, 0);
        assert_eq!(inverted, vec![Cs, E, F]);
    }

    #[test]
    fn inverting_by_an_inversion_pair() {
        let set: Set = vec![G, Af, B];
        let inverted = by_pair(&set, (G, B));
        assert_eq!(inverted, vec![G, Bf, B]);
    }
}
