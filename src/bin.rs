use forte::{invert_by_pair, Set};

fn main() {
    let s1: Set = vec![0, 2, 5];

    println!("{s1:?}\t{:?}", invert_by_pair(&s1, (1, 5)));
}
