use crate::{PitchClass, Set};

pub fn intervals(set: &Set) -> Vec<u32> {
    set.windows(2).map(find_interval).collect()
}

fn find_interval(pair: &[PitchClass]) -> u32 {
    let a = pair[0].to_u32();
    let b = pair[1].to_u32();
    get_interval_between(a, b)
}

pub fn get_interval_between(a: u32, b: u32) -> u32 {
    if b >= a {
        b - a
    } else {
        (b + 12) - a
    }
}
