use forte::{to_normal_form, Set};

fn main() {
    let s1: Set = vec![0, 2, 11];

    println!("{s1:?}\t{:?}", to_normal_form(&s1));
}
