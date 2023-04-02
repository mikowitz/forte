use forte::{to_normal_form, Set};

fn main() {
    let s1: Set = vec![1, 2, 5, 10];
    let s2: Set = vec![0, 5, 8, 11];
    let s3: Set = vec![3, 4, 6, 7];
    let s4: Set = vec![10, 1, 2, 6];

    println!("{s1:?}\t{:?}", to_normal_form(&s1));
    println!("{s2:?}\t{:?}", to_normal_form(&s2));
    println!("{s3:?}\t{:?}", to_normal_form(&s3));
    println!("{s4:?}\t{:?}", to_normal_form(&s4));
}
