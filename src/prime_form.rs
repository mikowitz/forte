use crate::{normal_form, utils, PitchClassSet};
use std::cmp::min;

pub fn from(set: &PitchClassSet) -> Vec<u32> {
    let nf: PitchClassSet = normal_form::from(set);
    let intervals: Vec<u32> = utils::intervals(nf.set());
    let mut reversed = intervals.clone();
    reversed.reverse();
    let mut left_packed = min(intervals, reversed);
    left_packed.insert(0, 0);
    left_packed
        .iter()
        .scan(0, |acc, interval| {
            *acc += interval;
            Some(*acc)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{from, PitchClassSet};
    use crate::{set, PitchClass::*};

    #[test]
    fn prime_form_from_normal_form() {
        let set: PitchClassSet = set![F, Fs, A];
        let prime = from(&set);
        assert_eq!(prime, vec![0, 1, 4]);
    }

    #[test]
    fn prime_form_from_non_normal_form() {
        let set: PitchClassSet = set![A, F, Fs];
        let prime = from(&set);
        assert_eq!(prime, vec![0, 1, 4]);
    }

    #[test]
    fn prime_form_from_right_packed_normal_form() {
        let set: PitchClassSet = set![Cs, F, Fs, G];
        let prime = from(&set);
        assert_eq!(prime, vec![0, 1, 2, 6]);
    }
}
