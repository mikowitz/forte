use forte::{PitchClass::*, Set};

fn main() {
    let s1: Set = vec![F, Af, A, Cs];

    println!("{s1:?}\t{:?}", forte::to_normal_form(&s1));
    println!("{s1:?}\t{:?}", forte::to_prime_form(&s1));
    // println!("{:?}, {:?}", A, A.to_u32());
}
