use crate::PitchClassSet;

/// Wrapper struct to represent the prime form of a T/I set class.
#[derive(Debug)]
pub struct SetClass {
    set: Vec<u32>,
}

impl SetClass {
    /// Constructs a set class from a given [PitchClassSet]
    pub fn new(pitch_class_set: &PitchClassSet) -> Self {
        let set = crate::prime_form::from(pitch_class_set);
        Self { set }
    }

    pub fn set(&self) -> &Vec<u32> {
        &self.set
    }
}

#[cfg(test)]
mod tests {
    use super::{PitchClassSet, SetClass};
    use crate::{set, PitchClass::*};

    #[test]
    fn new() {
        let pcs: PitchClassSet = set![Cs, F, Fs, G];
        let set_class = SetClass::new(&pcs);

        assert_eq!(set_class.set(), &[0, 1, 2, 6]);
    }
}
