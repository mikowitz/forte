use forte::PitchClass::*;
use forte::{i5, invert, invert_by_pair, t3, to_normal_form, to_prime_form, transpose, Set};

#[test]
fn transposition() {
    let set: Set = vec![G, Af, Bf, B];

    assert_eq!(transpose(&set, 3), vec![Bf, B, Cs, D]);
}

#[test]
fn generated_transposition_levels() {
    let set: Set = vec![G, Af, Bf, B];

    assert_eq!(t3(&set), vec![Bf, B, Cs, D]);
}

#[test]
fn inversion() {
    let set: Set = vec![Cs, Ef, E, G];

    assert_eq!(invert(&set, 5), vec![Bf, Cs, D, E]);
}

#[test]
fn inversion_by_pair() {
    let set: Set = vec![G, Af, B];

    assert_eq!(invert_by_pair(&set, (G, B)), vec![G, Bf, B]);
}

#[test]
fn generated_inversion_levels() {
    let set: Set = vec![Cs, Ef, E, G];

    assert_eq!(i5(&set), vec![Bf, Cs, D, E]);
}

#[test]
fn normal_form() {
    let set: Set = vec![A, Bf, F];
    let expected: Set = vec![F, A, Bf];
    assert_eq!(to_normal_form(&set), expected);
}

#[test]
fn prime_form() {
    let set: Set = vec![Bf, D, F, Fs];

    assert_eq!(to_prime_form(&set), vec![0, 1, 4, 8]);
}
