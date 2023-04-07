use forte::{i5, invert, invert_by_pair, t3, to_normal_form, to_prime_form, transpose};
use forte::{set, PitchClass::*, PitchClassSet};

#[test]
fn transposition() {
    let set: PitchClassSet = set![G, Af, Bf, B];

    assert_eq!(transpose(&set, 3), set![Bf, B, Cs, D]);
}

#[test]
fn generated_transposition_levels() {
    let set: PitchClassSet = set![G, Af, Bf, B];

    assert_eq!(t3(&set), set![Bf, B, Cs, D]);
}

#[test]
fn inversion() {
    let set: PitchClassSet = set![Cs, Ef, E, G];

    assert_eq!(invert(&set, 5), set![Bf, Cs, D, E]);
}

#[test]
fn inversion_by_pair() {
    let set: PitchClassSet = set![G, Af, B];

    assert_eq!(invert_by_pair(&set, (G, B)), set![G, Bf, B]);
}

#[test]
fn generated_inversion_levels() {
    let set: PitchClassSet = set![Cs, Ef, E, G];

    assert_eq!(i5(&set), set![Bf, Cs, D, E]);
}

#[test]
fn normal_form() {
    let set: PitchClassSet = set![A, Bf, F];
    let expected: PitchClassSet = set![F, A, Bf];
    assert_eq!(to_normal_form(&set), expected);
}

#[test]
fn prime_form() {
    let set: PitchClassSet = set![Bf, D, F, Fs];

    assert_eq!(to_prime_form(&set), vec![0, 1, 4, 8]);
}
