use crate::Set;

pub fn intervals(set: &Set) -> Vec<u32> {
    set.windows(2).map(find_interval).collect()
}

fn find_interval(pair: &[u32]) -> u32 {
    let a = pair[0];
    let b = pair[1];
    get_interval_between(a, b)
}

pub fn get_interval_between(a: u32, b: u32) -> u32 {
    if b >= a {
        b - a
    } else {
        (b + 12) - a
    }
}
