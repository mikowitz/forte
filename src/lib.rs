//! This crate provides functionality for interacting with and manipulating
//! pitch and pitch class sets.
//! ## Normal Form
//!
//! A set can be converted into its normal form using [to_normal_form]
//!
//! ```
//! # use forte::{Set, to_normal_form};
//! let set: Set = vec![0, 2, 11];
//! let normal = to_normal_form(&set);
//! assert_eq!(normal, vec![11, 0, 2]);
//! ```
//!
//! ## Prime Form
//!
//! A set can be converted to the prime form of its set class using [to_prime_form]
//!
//! ```
//! # use forte::{to_prime_form, Set};
//! let set: Set = vec![1, 5, 6, 7];
//! let prime = to_prime_form(&set);
//! assert_eq!(prime, vec![0,1,2,6]);
//! ```
//! ## Transposition
//!
//! `forte` provides two methods for transposing a set.
//!
//! First, via [transpose], which takes a reference to a set and an integer
//! (mod 12) specifying the transposition level
//!
//! ```rust
//! # use forte::{Set, transpose};
//! let set: Set = vec![0, 2, 5];
//! let transposed = transpose(&set, 3);
//! assert_eq!(transposed, vec![3, 5, 8]);
//! ```
//!
//! Second, by using `forte`'s `tX` functions, which tranpose a given set by `X`
//! ```rust
//! # use forte::Set;
//! let set: Set = vec![0, 2, 5];
//! let transposed = forte::t4(&set);
//! assert_eq!(transposed, vec![4, 6, 9]);
//! ```
//!
//! The available functions of this form are [t0], [t1], ... [t11]
//!
//! ## Inversion
//!
//! Similar to transposition, [forte](crate) provides two methods for inverting a set.
//!
//! First, via [invert], which takes a reference to a set and an integer
//! (mod 12) specifying the inversion level
//!
//! ```rust
//! # use forte::{Set, invert};
//! let set: Set = vec![0, 2, 5];
//! let inverted = invert(&set, 4);
//! assert_eq!(inverted, vec![11, 2, 4]);
//! ```
//! Second, by using [forte](crate)'s `iX` functions, [i0], [i1], ... [i11], which invert a given set around `X`
//! ```rust
//! # use forte::Set;
//! let set: Set = vec![0, 2, 5];
//! let inverted = forte::i3(&set);
//! assert_eq!(inverted, vec![10, 1, 3]);
//! ```
//! `forte` also provides [invert_by_pair], which takes a reference
//! to a set, and a 2-element tuple of integers (mod 12) that defines one of the
//! desired inversion level's pitch class mappings.
//!
//! ```rust
//! # use forte::{invert_by_pair, Set};
//! let set: Set = vec![0, 2, 5];
//! let inverted = invert_by_pair(&set, (1, 5));
//! assert_eq!(inverted, vec![1, 4, 6]);
//! ```

/// Syntatic alias to specify intent throughout [forte](crate)
pub type Set = Vec<u32>;

mod inversion;
mod normal_form;
mod prime_form;
mod transposition;
mod utils;

use paste::paste;

/// Reorders a pitch class set into normal form (the most compressed way of
/// representing the set).
///
/// Normal form ensures that the interval between the first and last
/// elements of the set is as small as possible.
///
/// ```
/// # use forte::{to_normal_form, Set};
/// let set: Set = vec![10, 5, 9]; // Total interval (mod 12) is 11
/// # let normal = to_normal_form(&set);
/// # assert_eq!(normal, vec![5, 9, 10]); // Total interval (mod 12) is 5
/// ```
/// By converting the set to normal form, we find a much more compact
/// ordering of the set elements
///
/// ```
/// # use forte::{to_normal_form, Set};
/// # let set: Set = vec![10, 5, 9]; // Total interval (mod 12) is 11
/// let normal = to_normal_form(&set);
/// assert_eq!(normal, vec![5, 9, 10]); // Total interval (mod 12) is 5
/// ```
///
/// ## Sets with multiple minimal total intervals
///
/// Some sets have more than one ordering with a minimal total interval
///
/// ```
/// # use forte::{to_normal_form, Set};
/// let set1: Set = vec![5, 8, 9, 1]; // Total interval (mod 12) is 8
/// let set2: Set = vec![1, 5, 8, 9]; // Total interval is *also* 8
/// # let normal = to_normal_form(&set1);
/// # assert_eq!(normal, set2);
/// ```
/// In these instances, [forte](`crate`) will look for an ordering that
/// has the smaller intervals packed towards the top or the bottom of the
/// normal form ordering.
///
/// * `set1` has intervals (mod 12) of `[3, 1, 4]`
/// * `set2` has intervals (mod 12) of `[4, 1, 3]`
///
/// In this case, `set2` has more closely packed intervals at the top of the ordered set,
/// so it is this set's normal form
/// ```
/// # use forte::{to_normal_form, Set};
/// # let set1: Set = vec![5, 8, 9, 1]; // Total interval (mod 12) is 8
/// # let set2: Set = vec![1, 5, 8, 9]; // Total interval is *also* 8
/// let normal = to_normal_form(&set1);
/// assert_eq!(normal, set2);
/// ```
///
/// ## Sets with multiple minimal total intervals and identical interval packings
///
/// In some cases (for example, inversionally symmetrical sets), not only will
/// there be multiple orderings with the same total interval size, but the
/// intervals for these orderings will be mirror images of each other.
/// ```
/// # use forte::{to_normal_form, Set};
/// let set1: Set = vec![4, 8, 9, 11, 0]; // Total interval (mod 12) is 8
/// let set2: Set = vec![8, 9, 11, 0, 4]; // Total interval is *also* 8
/// # let normal = to_normal_form(&set1);
/// # assert_eq!(normal, set2);
/// ```
/// * `set1` has intervals (mod 12) of `[4, 1, 2, 1]`
/// * `set2` has intervals (mod 12) of `[1, 2, 1, 4]`
///
/// When two possible normal forms have the same (mirrored) interval content,
/// [forte](`crate`) will choose the ordering with smaller intervals packed
/// towards the bottom of the ordering.
///
/// In this case, `set2` is packed more towards the bottom, so it is chosen
/// as the normal form of this pitch class set.
/// ```
/// # use forte::{to_normal_form, Set};
/// # let set1: Set = vec![4, 8, 9, 11, 0]; // Total interval (mod 12) is 8
/// # let set2: Set = vec![8, 9, 11, 0, 4]; // Total interval is *also* 8
/// let normal = to_normal_form(&set1);
/// assert_eq!(normal, set2);
/// ```
pub fn to_normal_form(set: &Set) -> Set {
    normal_form::from(set)
}

/// Converts a pitch class set to its set class prime form.
///
/// Similar to normal form, the prime form represents the best-packed
/// ordering of the set, but favors packing to the left. In addition,
/// prime forms always begin on 0, as they represent the base
/// form for the entire T/I set class.
/// ```
/// # use forte::{to_prime_form, Set};
/// let set: Set = vec![10, 5, 9]; // Total interval (mod 12) is 11
/// # let prime = to_prime_form(&set);
/// # assert_eq!(prime, vec![0, 1, 5]);
/// ```
/// Unlike in [normal form](to_normal_form), where the reorderd set contains the original pitch class data,
/// in prime form, the set is reordered and transposed to begin on `0`.
/// ```
/// # use forte::{to_prime_form, Set};
/// # let set: Set = vec![10, 5, 9]; // Total interval (mod 12) is 11
/// let prime = to_prime_form(&set);
/// assert_eq!(prime, vec![0, 1, 5]); // Total interval (mod 12) is 5, and begins on 0
/// ```
/// Likewise, because the prime form is the base form of the T/I set class to which the pitch
/// class set belongs, multiple pitch class sets will have the same prime form
/// ```
/// # use forte::{to_prime_form, Set};
/// let set: Set = vec![4, 3, 8];
/// let prime = to_prime_form(&set);
/// assert_eq!(prime, vec![0, 1, 5]);
/// ```
pub fn to_prime_form(set: &Set) -> Set {
    prime_form::from(set)
}

/// Transposes a pitch class set by a given level (mod 12)
///
/// The calculation for transposition
/// ```markdown
/// If Tn(x) = y, then y = x + n. So y - x = n.
/// ```
///
/// ```
/// # use forte::{transpose, Set};
/// let set: Set = vec![4, 5, 8];
/// let transposed = transpose(&set, 5);
/// assert_eq!(transposed, vec![9, 10, 1]);
/// ```
/// Transposition can also be performed by the `tX` functions: [t0], [t1], ... [t11]
/// ```
/// # use forte::{Set};
/// let set: Set = vec![4, 5, 8];
/// let transposed = forte::t3(&set);
/// assert_eq!(transposed, vec![7, 8, 11]);
pub fn transpose(set: &Set, level: u32) -> Set {
    transposition::by(set, level)
}

/// Inverts a pitch class set around a given level (mod 12)
///
/// The calculation for inversion
/// ```markdown
/// If In(x) = y, then y = n - x. So x + y = n.
/// ```
///
/// ```
/// # use forte::{invert, Set};
/// let set: Set = vec![4, 5, 8];
/// let inverted = invert(&set, 5);
/// assert_eq!(inverted, vec![9, 0, 1]);
/// ```
/// Inversion can also be performed by the `iX` functions: [i0], [i1], ... [i11]
/// ```
/// # use forte::{Set};
/// let set: Set = vec![4, 5, 8];
/// let inverted = forte::i0(&set);
/// assert_eq!(inverted, vec![4, 7, 8]);
/// ```
/// **NB** As shown above, unlike transposition, where transposing by 0 is an identity
/// function (that is, it returns its input), inverting around 0 is *not* an identity
/// function.
pub fn invert(set: &Set, level: u32) -> Set {
    inversion::by(set, level)
}

/// Invert a set by specifying one pair of pitch classes that map onto each
/// other by the desired inversion level.
/// ```
/// # use forte::{invert_by_pair, Set};
/// let set: Set = vec![0, 1, 2, 5];
/// let inverted = invert_by_pair(&set, (0, 1));
/// assert_eq!(inverted, vec![8, 11, 0, 1]);
/// ```
/// In this example, the set is inverted around a level that results in `0` being inverted to `1`
/// and vice versa.
///
/// As per the equation for inversion
/// ```markdown
/// If In(x) = y, then y = n - x. So x + y = n.
/// ```
/// we know that `In(0) = 1`, and so `n = 0 + 1`.
///
/// Thus, this inversion is equivalent to calling
/// ```
/// # use forte::{invert, Set};
/// # let set: Set = vec![0, 1, 2, 5];
/// invert(&set, 1);
/// ```
/// or
/// ```
/// # use forte::{Set};
/// # let set: Set = vec![0, 1, 2, 5];
/// forte::i1(&set);
/// ```
pub fn invert_by_pair(set: &Set, inversion_pair: (u32, u32)) -> Set {
    inversion::by_pair(set, inversion_pair)
}

define_transpositions!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
define_inversions!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);

#[cfg(test)]
mod tests {
    use crate::{i5, invert, invert_by_pair, t3, to_prime_form, transpose, Set};

    #[test]
    fn transposition() {
        let set: Set = vec![7, 8, 10, 11];

        assert_eq!(transpose(&set, 3), vec![10, 11, 1, 2]);
    }

    #[test]
    fn generated_transposition_levels() {
        let set: Set = vec![7, 8, 10, 11];

        assert_eq!(t3(&set), vec![10, 11, 1, 2]);
    }

    #[test]
    fn inversion() {
        let set: Set = vec![1, 3, 4, 7];

        assert_eq!(invert(&set, 5), vec![10, 1, 2, 4]);
    }

    #[test]
    fn inversion_by_pair() {
        let set: Set = vec![7, 8, 11];

        assert_eq!(invert_by_pair(&set, (7, 11)), vec![7, 10, 11]);
    }

    #[test]
    fn generated_inversion_levels() {
        let set: Set = vec![1, 3, 4, 7];

        assert_eq!(i5(&set), vec![10, 1, 2, 4]);
    }

    #[test]
    fn to_normal_form() {
        let set: Set = vec![10, 2, 5, 6];

        assert_eq!(to_prime_form(&set), vec![0, 1, 4, 8]);
    }
}
