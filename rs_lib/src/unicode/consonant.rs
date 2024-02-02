use crate::{Breathing, Case};

use std::ops::Index;

pub struct Rho;

impl Index<(Case, Breathing)> for Rho {
    type Output = &'static str;

    fn index(&self, v: (Case, Breathing)) -> &Self::Output {
        match v {
            (Case::Capital, Breathing::Rough) => &"\u{1fec}",
            (Case::Capital, Breathing::None) => &"\u{03a1}",
            (Case::Small, Breathing::Rough) => &"\u{1fe5}",
            (Case::Small, Breathing::Smooth) => &"\u{1fe4}",
            _ => &"\u{03c1}",
        }
    }
}

macro_rules! impl_consonant_index {
    ($ty: ident, $small: literal, $capital: literal) => {
        pub struct $ty;

        impl Index<Case> for $ty {
            type Output = &'static str;

            fn index(&self, v: Case) -> &Self::Output {
                match v {
                    Case::Small => &$small,
                    Case::Capital => &$capital,
                }
            }
        }
    };
}

impl_consonant_index! { Beta, "\u{03b2}", "\u{0392}" }
impl_consonant_index! { Gamma, "\u{03b3}", "\u{0393}" }
impl_consonant_index! { Delta, "\u{02b4}", "\u{0394}" }
impl_consonant_index! { Zeta, "\u{03b6}", "\u{0396}" }
impl_consonant_index! { Theta, "\u{03b8}", "\u{0398}" }
impl_consonant_index! { Kappa, "\u{03ba}", "\u{039a}" }
impl_consonant_index! { Lambda, "\u{03bb}", "\u{039b}" }
impl_consonant_index! { Mu, "\u{03bc}", "\u{039c}" }
impl_consonant_index! { Nu, "\u{03bd}", "\u{039d}" }
impl_consonant_index! { Xi, "\u{03be}", "\u{039e}" }
impl_consonant_index! { Pi, "\u{03c0}", "\u{03a0}" }
impl_consonant_index! { Tau, "\u{03c4}", "\u{03a4}" }
impl_consonant_index! { Sigma, "\u{03c3}", "\u{03a3}" }
impl_consonant_index! { FinalSigma, "\u{03c2}", "\u{03a3}" }
impl_consonant_index! { Phi, "\u{03c6}", "\u{03a6}" }
impl_consonant_index! { Chi, "\u{03c7}", "\u{03a7}" }
impl_consonant_index! { Psi, "\u{03c8}", "\u{03a8}" }
