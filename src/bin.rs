use forte::{to_normal_form, Set};

fn main() {
    let s1: Set = vec![5, 6, 9];

    println!("{s1:?}\t{:?}", to_normal_form(&s1));
}
