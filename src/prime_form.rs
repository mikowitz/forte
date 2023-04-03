use crate::{normal_form, utils, Set};
use std::cmp::min;

pub fn from(set: &Set) -> Set {
    let nf: Set = normal_form::from(set);
    let intervals: Vec<u32> = utils::intervals(&nf);
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
    use super::{from, Set};

    #[test]
    fn prime_form_from_normal_form() {
        let set: Set = vec![5, 6, 9];
        let prime = from(&set);
        assert_eq!(prime, vec![0, 1, 4]);
    }

    #[test]
    fn prime_form_from_non_normal_form() {
        let set: Set = vec![9, 5, 6];
        let prime = from(&set);
        assert_eq!(prime, vec![0, 1, 4]);
    }

    #[test]
    fn prime_form_from_right_packed_normal_form() {
        let set: Set = vec![1, 5, 6, 7];
        let prime = from(&set);
        assert_eq!(prime, vec![0, 1, 2, 6]);
    }
}
