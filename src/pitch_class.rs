/// Representation of pitch classes in a 12-EDO tonal system.
///
/// All 7 diatonic pitch classes (C, D, E, F, G, A B) are defined,
/// as well as their sharp and flat alterations. Double sharp and
/// double flat alterations are included as well, with the exceptions
/// of C𝄫, E𝄪, F𝄫, and B𝄪, as these alterations modify the diatonic
/// pitch class by more than one step.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PitchClass {
    /// C♭ (enharmonically equivalent to [B♮](PitchClass::B))
    Cf,
    /// C♮
    C,
    /// C♯
    Cs,
    /// C𝄪 (enharmonically equivalent to [D♮](PitchClass::D))
    Css,

    /// D𝄫 (enharmonically equivalent to [C♮](PitchClass::C))
    Dff,
    /// D♭
    Df,
    /// D♮
    D,
    /// D♯
    Ds,
    /// D𝄪 (enharmonically equivalent to [E♮](PitchClass::E))
    Dss,

    /// E𝄫 (enharmonically equivalent to [D♮](PitchClass::D))
    Eff,
    /// E♭
    Ef,
    /// E♮
    E,
    /// E♯ (enharmonically equivalent to [F♮](PitchClass::F))
    Es,

    /// F♭ (enharmonically equivalent to [E♮](PitchClass::E))
    Ff,
    /// F♮
    F,
    /// F♯
    Fs,
    /// F𝄪 (enharmonically equivalent to [G♮](PitchClass::G))
    Fss,

    /// G𝄫 (enharmonically equivalent to [F♮](PitchClass::F))
    Gff,
    /// G♭
    Gf,
    /// G♮
    G,
    /// G♯
    Gs,
    /// G𝄪 (enharmonically equivalent to [A♮](PitchClass::A))
    Gss,

    /// A𝄫 (enharmonically equivalent to [G♮](PitchClass::G))
    Aff,
    /// A♭
    Af,
    /// A♮
    A,
    /// A♯
    As,
    /// A𝄪 (enharmonically equivalent to [B♮](PitchClass::B))
    Ass,

    /// B𝄫 (enharmonically equivalent to [A♮](PitchClass::A))
    Bff,
    /// B♭
    Bf,
    /// B♮
    B,
    /// B♯ (enharmonically equivalent to [C♮](PitchClass::C))
    Bs,
}

impl PitchClass {
    /// Converts a [PitchClass] to its numeric equivalent (mod 12)
    ///
    /// As is common in (Western) music theory, C♮ is assigned a numeric
    /// value of 0, and each semitone is an increment of 1.
    /// ```
    /// # use forte::{PitchClass::*};
    /// assert_eq!(C.to_u32(), 0);
    /// ```
    pub fn to_u32(self) -> u32 {
        self.into()
    }
}

use PitchClass::*;
impl From<PitchClass> for u32 {
    fn from(value: PitchClass) -> u32 {
        match value {
            Bs | C | Dff => 0,
            Cs | Df => 1,
            Css | D | Eff => 2,
            Ds | Ef => 3,
            Dss | E | Ff => 4,
            Es | F | Gff => 5,
            Fs | Gf => 6,
            Fss | G | Aff => 7,
            Gs | Af => 8,
            Gss | A | Bff => 9,
            As | Bf => 10,
            Ass | B | Cf => 11,
        }
    }
}

impl From<u32> for PitchClass {
    fn from(value: u32) -> Self {
        match value.rem_euclid(12) {
            0 => C,
            1 => Cs,
            2 => D,
            3 => Ef,
            4 => E,
            5 => F,
            6 => Fs,
            7 => G,
            8 => Af,
            9 => A,
            10 => Bf,
            11 => B,
            x => panic!("rem_euclid(12) should never return {x}"),
        }
    }
}
