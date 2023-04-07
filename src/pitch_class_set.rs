use crate::PitchClass;

/// Wrapper struct for a list of [PitchClasses](PitchClass).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PitchClassSet {
    set: Vec<PitchClass>,
}

impl PitchClassSet {
    /// Create a new [PitchClassSet] from a list of [PitchClasses](PitchClass)
    ///
    /// The macro [forte::set!](`crate::set!`) provides a wrapper for this function, and in general
    /// should be prefered for creating new [PitchClassSet] instances.
    pub fn new(set: Vec<PitchClass>) -> Self {
        Self { set }
    }

    /// Returns a reference to the contained pitch class set.
    ///
    /// ```
    /// # use forte::{PitchClassSet, PitchClass::*, set};
    /// let set: PitchClassSet = set![C, D, E];
    /// assert_eq!(set.set(), &[C, D, E]);
    /// ```
    pub fn set(&self) -> &Vec<PitchClass> {
        &self.set
    }
}
