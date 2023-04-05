use crate::{utils, Set};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Internal {
    set: Set,
    intervals: Vec<u32>,
    total_span: u32,
}

impl Internal {
    pub fn new(set: Set) -> Self {
        let total_span = total_span(&set);
        let intervals = utils::intervals(&set);
        Self {
            set,
            intervals,
            total_span,
        }
    }
}

fn total_span(set: &Set) -> u32 {
    let a = set.first().unwrap();
    let b = set.last().unwrap();
    utils::get_interval_between((*a).to_u32(), (*b).to_u32())
}

pub fn from(set: &Set) -> Set {
    let mut set = set.clone();
    set.sort_unstable();
    let rotations: Vec<Internal> = (1..=set.len())
        .map(|i| Internal::new(create_rotation(&set, i)))
        .collect();

    let min_span = rotations.iter().map(|r| r.total_span).min().unwrap();

    let rotations_with_min_span: Vec<&Internal> = rotations
        .iter()
        .filter(|set| set.total_span == min_span)
        .collect();

    find_normal_form(rotations_with_min_span).set.clone()
}

fn create_rotation(set: &Set, rotation: usize) -> Set {
    let mut rotated = set.clone();
    rotated.rotate_right(rotation);
    rotated.to_vec()
}

fn find_normal_form(candidates: Vec<&Internal>) -> &Internal {
    if candidates.len() == 1 {
        candidates[0]
    } else {
        find_best_packed_candidate(candidates)
    }
}

fn find_best_packed_candidate(sets: Vec<&Internal>) -> &Internal {
    let mut all_interval_possibilities: Vec<(Vec<u32>, bool, &Internal)> = vec![];

    for set in &sets {
        all_interval_possibilities.push(set_with_interval_scan(set, false));
        all_interval_possibilities.push(set_with_interval_scan(set, true));
    }
    all_interval_possibilities.sort();
    all_interval_possibilities[0].2
}

fn set_with_interval_scan(set: &Internal, is_reversed: bool) -> (Vec<u32>, bool, &Internal) {
    let mut intervals = set.intervals.clone();
    if is_reversed {
        intervals.reverse();
    }
    let interval_scan: Vec<u32> = create_interval_scan(intervals);
    (interval_scan, is_reversed, set)
}

fn create_interval_scan(intervals: Vec<u32>) -> Vec<u32> {
    intervals
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
    use crate::PitchClass::*;

    #[test]
    fn normal_form_with_one_clear_answer() {
        let set: Set = vec![A, Bf, F];
        let expected: Set = vec![F, A, Bf];
        assert_eq!(from(&set), expected);
    }

    #[test]
    fn normal_form_with_multiple_options_with_same_span_but_different_packing() {
        let set: Set = vec![F, Af, A, Cs];
        let expected: Set = vec![Cs, F, Af, A];
        assert_eq!(from(&set), expected);
    }

    #[test]
    fn normal_form_prefers_packing_to_the_left() {
        let set: Set = vec![C, E, Af, A, B];
        let expected: Set = vec![Af, A, B, C, E];
        assert_eq!(from(&set), expected);
    }
}
