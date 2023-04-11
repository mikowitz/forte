//! This crate provides functionality for interacting with and manipulating
//! pitch and pitch class sets.
//! ## Normal Form
//!
//! A set can be converted into its normal form using [to_normal_form]
//!
//! ```
//! # use forte::{PitchClassSet, PitchClass::*, to_normal_form, set};
//! let set: PitchClassSet = set![C, D, B];
//! let normal = to_normal_form(&set);
//! assert_eq!(normal, set![B, C, D]);
//! ```
//!
//! ## Prime Form
//!
//! A set can be converted to the prime form of its set class using [to_prime_form]
//!
//! ```
//! # use forte::{PitchClassSet, set, to_prime_form, PitchClass::*};
//! let set: PitchClassSet = set![Cs, F, Fs, G];
//! let prime = to_prime_form(&set);
//! assert_eq!(prime.set(), &[0, 1, 2, 6]);
//! ```
//! ## Transposition
//!
//! `forte` provides two methods for transposing a set.
//!
//! First, via [transpose], which takes a reference to a set and an integer
//! (mod 12) specifying the transposition level
//!
//! ```rust
//! # use forte::{PitchClassSet, set, PitchClass::*, transpose};
//! let set: PitchClassSet = set![C, D, F];
//! let transposed = transpose(&set, 3);
//! assert_eq!(transposed, set![Ef, F, Af]);
//! ```
//!
//! Second, by using `forte`'s `tX` functions, which tranpose a given set by `X`
//! ```rust
//! # use forte::{PitchClass::*, set, PitchClassSet};
//! let set: PitchClassSet = set![C, D, F];
//! let transposed = forte::t4(&set);
//! assert_eq!(transposed, set![E, Fs, A]);
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
//! # use forte::{PitchClassSet, set, PitchClass::*, invert};
//! let set: PitchClassSet = set![C, D, F];
//! let inverted = invert(&set, 4);
//! assert_eq!(inverted, set![B, D, E]);
//! ```
//! Second, by using [forte](crate)'s `iX` functions, [i0], [i1], ... [i11], which invert a given set around `X`
//! ```rust
//! # use forte::{set, PitchClassSet, PitchClass::*};
//! let set: PitchClassSet = set![C, D, F];
//! let inverted = forte::i3(&set);
//! assert_eq!(inverted, set![Bf, Cs, Ef]);
//! ```
//! `forte` also provides [invert_by_pair], which takes a reference
//! to a set, and a 2-element tuple of integers (mod 12) that defines one of the
//! desired inversion level's pitch class mappings.
//!
//! ```rust
//! # use forte::{invert_by_pair, PitchClassSet, set, PitchClass::*};
//! let set: PitchClassSet = set![C, D, F];
//! let inverted = invert_by_pair(&set, (Cs, F));
//! assert_eq!(inverted, set![Cs, E, Fs]);
//! ```

mod inversion;
mod normal_form;
mod pitch_class;
mod pitch_class_set;
mod prime_form;
mod set_class;
mod transposition;
mod utils;

pub use pitch_class::PitchClass;
pub use pitch_class_set::PitchClassSet;
pub use set_class::SetClass;

use paste::paste;

/// Syntactic shorthand for creating a new [PitchClassSet]
/// ```
/// use forte::{PitchClassSet, PitchClass::*, set};
/// let set: PitchClassSet = set![C, E, G];
/// ```
#[macro_export]
macro_rules! set {
    (
        $($pc:expr) , *
    ) => {
        PitchClassSet::new(vec![$($pc),*])
    }
}

/// Reorders a pitch class set into normal form (the most compressed way of
/// representing the set).
///
/// Normal form ensures that the interval between the first and last
/// elements of the set is as small as possible.
///
/// ```
/// # use forte::{to_normal_form, set, PitchClassSet, PitchClass::*};
/// let set: PitchClassSet = set![Bf, F, A]; // Total interval (mod 12) is 11
/// # let normal = to_normal_form(&set);
/// # assert_eq!(normal, set![F, A, Bf]); // Total interval (mod 12) is 5
/// ```
/// By converting the set to normal form, we find a much more compact
/// ordering of the set elements
///
/// ```
/// # use forte::{to_normal_form, set, PitchClassSet, PitchClass::*};
/// # let set: PitchClassSet = set![Bf, F, A]; // Total interval (mod 12) is 11
/// let normal = to_normal_form(&set);
/// assert_eq!(normal, set![F, A, Bf]); // Total interval (mod 12) is 5
/// ```
///
/// ## Sets with multiple minimal total intervals
///
/// Some sets have more than one ordering with a minimal total interval
///
/// ```
/// # use forte::{to_normal_form, set, PitchClassSet, PitchClass::*};
/// let set1: PitchClassSet = set![F, Af, A, Cs]; // Total interval (mod 12) is 8
/// let set2: PitchClassSet = set![Cs, F, Af, A]; // Total interval is *also* 8
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
/// # use forte::{to_normal_form, set, PitchClassSet, PitchClass::*};
/// # let set1: PitchClassSet = set![F, Af, A, Cs]; // Total interval (mod 12) is 8
/// # let set2: PitchClassSet = set![Cs, F, Af, A]; // Total interval is *also* 8
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
/// # use forte::{to_normal_form, set, PitchClassSet, PitchClass::*};
/// let set1: PitchClassSet = set![E, Af, A, B, C]; // Total interval (mod 12) is 8
/// let set2: PitchClassSet = set![Af, A, B, C, E]; // Total interval is *also* 8
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
/// # use forte::{to_normal_form, PitchClassSet, set, PitchClass::*};
/// # let set1: PitchClassSet = set![E, Af, A, B, C]; // Total interval (mod 12) is 8
/// # let set2: PitchClassSet = set![Af, A, B, C, E]; // Total interval is *also* 8
/// let normal = to_normal_form(&set1);
/// assert_eq!(normal, set2);
/// ```
pub fn to_normal_form(set: &PitchClassSet) -> PitchClassSet {
    normal_form::from(set)
}

/// Converts a pitch class set to its set class prime form.
///
/// Similar to normal form, the prime form represents the best-packed
/// ordering of the set, but favors packing to the left. In addition,
/// prime forms always begin on 0, as they represent the base
/// form for the entire T/I set class.
/// ```
/// # use forte::{to_prime_form, set, PitchClassSet, PitchClass::*};
/// let set: PitchClassSet = set![Bf, F, A]; // Total interval (mod 12) is 11
/// # let prime = to_prime_form(&set);
/// # assert_eq!(prime.set(), &[0, 1, 5]);
/// ```
/// Unlike in [normal form](to_normal_form), where the reorderd set contains the original pitch class data,
/// in prime form, the set is reordered and transposed to begin on `0`.
/// ```
/// # use forte::{to_prime_form, set, PitchClassSet, PitchClass::*};
/// # let set: PitchClassSet = set![Bf, F, A]; // Total interval (mod 12) is 11
/// let prime = to_prime_form(&set);
/// assert_eq!(prime.set(), &[0, 1, 5]); // Total interval (mod 12) is 5, and begins on 0
/// ```
/// Likewise, because the prime form is the base form of the T/I set class to which the pitch
/// class set belongs, multiple pitch class sets will have the same prime form
/// ```
/// # use forte::{to_prime_form, set, PitchClassSet, PitchClass::*};
/// let set: PitchClassSet = set![E, Ef, Af];
/// let prime = to_prime_form(&set);
/// assert_eq!(prime.set(), &[0, 1, 5]);
/// ```
pub fn to_prime_form(set: &PitchClassSet) -> SetClass {
    SetClass::new(set)
}

/// Transposes a pitch class set by a given level (mod 12)
///
/// The calculation for transposition
/// ```markdown
/// If Tn(x) = y, then y = x + n. So y - x = n.
/// ```
///
/// ```
/// # use forte::{transpose, set, PitchClassSet, PitchClass::*};
/// let set: PitchClassSet = set![E, F, Af];
/// let transposed = transpose(&set, 5);
/// assert_eq!(transposed, set![A, Bf, Cs]);
/// ```
/// Transposition can also be performed by the `tX` functions: [t0], [t1], ... [t11]
/// ```
/// # use forte::{set, PitchClassSet, PitchClass::*};
/// let set: PitchClassSet = set![E, F, Af];
/// let transposed = forte::t3(&set);
/// assert_eq!(transposed, set![G, Af, B]);
pub fn transpose(set: &PitchClassSet, level: u32) -> PitchClassSet {
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
/// # use forte::{invert, set, PitchClassSet, PitchClass::*};
/// let set: PitchClassSet = set![E, F, Af];
/// let inverted = invert(&set, 5);
/// assert_eq!(inverted, set![A, C, Cs]);
/// ```
/// Inversion can also be performed by the `iX` functions: [i0], [i1], ... [i11]
/// ```
/// # use forte::{PitchClassSet, set, PitchClass::*};
/// let set: PitchClassSet = set![E, F, Af];
/// let inverted = forte::i0(&set);
/// assert_eq!(inverted, set![E, G, Af]);
/// ```
/// **NB** As shown above, unlike transposition, where transposing by 0 is an identity
/// function (that is, it returns its input), inverting around 0 is *not* an identity
/// function.
pub fn invert(set: &PitchClassSet, level: u32) -> PitchClassSet {
    inversion::by(set, level)
}

/// Invert a set by specifying one pair of pitch classes that map onto each
/// other by the desired inversion level.
/// ```
/// # use forte::{invert_by_pair, PitchClassSet, set, PitchClass::*};
/// let set: PitchClassSet = set![C, Cs, D, F];
/// let inverted = invert_by_pair(&set, (C, Cs));
/// assert_eq!(inverted, set![Af, B, C, Cs]);
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
/// # use forte::{invert, PitchClassSet, set, PitchClass::*};
/// # let set: PitchClassSet = set![C, Cs, D, F];
/// invert(&set, 1);
/// ```
/// or
/// ```
/// # use forte::{PitchClassSet, set, PitchClass::*};
/// # let set: PitchClassSet = set![C, Cs, D, F];
/// forte::i1(&set);
/// ```
pub fn invert_by_pair(
    set: &PitchClassSet,
    inversion_pair: (PitchClass, PitchClass),
) -> PitchClassSet {
    inversion::by_pair(set, inversion_pair)
}

define_transpositions!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
define_inversions!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
