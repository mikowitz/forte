use forte::Set;

fn main() {
    let s1: Set = vec![5, 8, 9, 1];

    println!("{s1:?}\t{:?}", forte::to_normal_form(&s1));
    println!("{s1:?}\t{:?}", forte::to_prime_form(&s1));
}
