use crate::PitchClassSet;

/// Wrapper struct to represent the prime form of a T/I set class.
#[derive(Debug)]
pub struct SetClass {
    set: Vec<u32>,
}

impl SetClass {
    pub fn set(&self) -> &Vec<u32> {
        &self.set
    }
}

impl From<&PitchClassSet> for SetClass {
    fn from(value: &PitchClassSet) -> Self {
        let set = crate::prime_form::from(value);
        Self { set }
    }
}

#[cfg(test)]
mod tests {
    use super::{PitchClassSet, SetClass};
    use crate::{set, PitchClass::*};

    #[test]
    fn from() {
        let pcs: PitchClassSet = set![Cs, F, Fs, G];
        let set_class = SetClass::from(&pcs);

        assert_eq!(set_class.set(), &[0, 1, 2, 6]);
    }
}
