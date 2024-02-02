use crate::{Accent, Breathing, Case, Diaeresis, Subscript};

use std::ops::Index;

fn breathing_idx(v: Breathing) -> usize {
    match v {
        Breathing::Smooth => 4,
        Breathing::Rough => 8,
        _ => 0,
    }
}
fn accent_idx(v: Accent) -> usize {
    match v {
        Accent::Grave => 1,
        Accent::Acute => 2,
        Accent::Circumflex => 3,
        _ => 0,
    }
}
fn subscript_idx(v: Subscript) -> usize {
    if matches!(v, Subscript::Iota) {
        12
    } else {
        0
    }
}
fn diaeresis_idx(v: Diaeresis) -> usize {
    if matches!(v, Diaeresis::Some) {
        12
    } else {
        0
    }
}

macro_rules! impl_vowel_index {
    ($ty: ident, (Breathing, Accent, Subscript), $small: expr, $capital: expr) => {
        pub struct $ty;

        impl Index<(Case, Breathing, Accent, Subscript)> for $ty {
            type Output = &'static str;

            fn index(&self, v: (Case, Breathing, Accent, Subscript)) -> &Self::Output {
                let idx = breathing_idx(v.1) + accent_idx(v.2) + subscript_idx(v.3);

                match v.0 {
                    Case::Small => &$small[idx],
                    Case::Capital => &$capital[idx],
                }
            }
        }
    };
    ($ty: ident, (Breathing, Accent, Diaeresis), $small: expr, $capital: expr) => {
        pub struct $ty;

        impl Index<(Case, Breathing, Accent, Diaeresis)> for $ty {
            type Output = &'static str;

            fn index(&self, v: (Case, Breathing, Accent, Diaeresis)) -> &Self::Output {
                let idx = breathing_idx(v.1) + accent_idx(v.2) + diaeresis_idx(v.3);

                match v.0 {
                    Case::Small => &$small[idx],
                    Case::Capital => &$capital[idx],
                }
            }
        }
    };
    ($ty: ident, (Breathing, Accent), $small: expr, $capital: expr) => {
        pub struct $ty;

        impl Index<(Case, Breathing, Accent)> for $ty {
            type Output = &'static str;

            fn index(&self, v: (Case, Breathing, Accent)) -> &Self::Output {
                let idx = breathing_idx(v.1) + accent_idx(v.2);

                match v.0 {
                    Case::Small => &$small[idx],
                    Case::Capital => &$capital[idx],
                }
            }
        }
    };
}

impl_vowel_index! { Alpha, (Breathing, Accent, Subscript), SMALL_ALPHA, CAP_ALPHA }
impl_vowel_index! { Epsilon, (Breathing, Accent), SMALL_EPSILON, CAP_EPSILON }
impl_vowel_index! { Eta, (Breathing, Accent, Subscript), SMALL_ETA, CAP_ETA }
impl_vowel_index! { Iota, (Breathing, Accent, Diaeresis), SMALL_IOTA, CAP_IOTA }
impl_vowel_index! { Omicron, (Breathing, Accent), SMALL_OMICRON, CAP_OMICRON }
impl_vowel_index! { Ypsilon, (Breathing, Accent, Diaeresis), SMALL_YPSILON, CAP_YPSILON }
impl_vowel_index! { Omega, (Breathing, Accent, Subscript), SMALL_OMEGA, CAP_OMEGA }

const SMALL_ALPHA: &[&str] = &[
    "\u{03b1}", "\u{1f70}", "\u{03ac}", "\u{1fb6}", "\u{1f00}", "\u{1f02}", "\u{1f04}", "\u{1f06}",
    "\u{1f01}", "\u{1f03}", "\u{1f05}", "\u{1f07}", "\u{1fb3}", "\u{1fb2}", "\u{1fb4}", "\u{1fb7}",
    "\u{1f80}", "\u{1f82}", "\u{1f84}", "\u{1f86}", "\u{1f81}", "\u{1f83}", "\u{1f85}", "\u{1f87}",
];
const SMALL_EPSILON: &[&str] = &[
    "\u{03b5}", "\u{1f72}", "\u{03ad}", "\u{03b5}", "\u{1f10}", "\u{1f12}", "\u{1f14}", "\u{1f10}",
    "\u{1f11}", "\u{1f13}", "\u{1f15}", "\u{1f11}",
];
const SMALL_ETA: &[&str] = &[
    "\u{03b7}", "\u{1f74}", "\u{03ae}", "\u{1fc6}", "\u{1f20}", "\u{1f22}", "\u{1f24}", "\u{1f26}",
    "\u{1f21}", "\u{1f23}", "\u{1f25}", "\u{1f27}", "\u{1fc3}", "\u{1fc2}", "\u{1fc4}", "\u{1fc7}",
    "\u{1f90}", "\u{1f92}", "\u{1f94}", "\u{1f96}", "\u{1f91}", "\u{1f93}", "\u{1f95}", "\u{1f97}",
];
const SMALL_IOTA: &[&str] = &[
    "\u{03b9}",
    "\u{1f76}",
    "\u{03af}",
    "\u{1fd6}",
    "\u{1f30}",
    "\u{1f32}",
    "\u{1f34}",
    "\u{1f36}",
    "\u{1f31}",
    "\u{1f33}",
    "\u{1f35}",
    "\u{1f37}",
    "\u{03ca}",
    "\u{1fd2}",
    "\u{0390}",
    "\u{1fd7}",
    "\u{03ca}\u{1fbd}",
    "\u{1fd2}\u{1fbd}",
    "\u{0390}\u{1fbd}",
    "\u{1fd7}\u{1fbd}",
    "\u{03ca}\u{1ffe}",
    "\u{1fd2}\u{1ffe}",
    "\u{0390}\u{1ffe}",
    "\u{1fd7}\u{1ffe}",
];
const SMALL_OMICRON: &[&str] = &[
    "\u{03bf}", "\u{1f78}", "\u{03cc}", "\u{03bf}", "\u{1f40}", "\u{1f42}", "\u{1f44}", "\u{1f40}",
    "\u{1f41}", "\u{1f43}", "\u{1f45}", "\u{1f41}",
];
const SMALL_YPSILON: &[&str] = &[
    "\u{03c5}",
    "\u{1f7a}",
    "\u{03cd}",
    "\u{1fe6}",
    "\u{1f50}",
    "\u{1f52}",
    "\u{1f54}",
    "\u{1f56}",
    "\u{1f51}",
    "\u{1f53}",
    "\u{1f55}",
    "\u{1f57}",
    "\u{03cb}",
    "\u{1fe2}",
    "\u{03b0}",
    "\u{1fe7}",
    "\u{03cb}\u{1fbd}",
    "\u{1fe2}\u{1fbd}",
    "\u{03b0}\u{1fbd}",
    "\u{1fe7}\u{1fbd}",
    "\u{03cb}\u{1ffe}",
    "\u{1fe2}\u{1ffe}",
    "\u{03b0}\u{1ffe}",
    "\u{1fe7}\u{1ffe}",
];
const SMALL_OMEGA: &[&str] = &[
    "\u{03c9}", "\u{1f7c}", "\u{03ce}", "\u{1ff6}", "\u{1f60}", "\u{1f62}", "\u{1f64}", "\u{1f66}",
    "\u{1f61}", "\u{1f63}", "\u{1f65}", "\u{1f67}", "\u{1ff3}", "\u{1ff2}", "\u{1ff4}", "\u{1ff7}",
    "\u{1fa0}", "\u{1fa2}", "\u{1fa4}", "\u{1fa6}", "\u{1fa1}", "\u{1fa3}", "\u{1fa5}", "\u{1fa7}",
];

const CAP_ALPHA: &[&str] = &[
    "\u{0391}",
    "\u{1fba}",
    "\u{0386}",
    "\u{1fc0}\u{0391}",
    "\u{1f08}",
    "\u{1f0a}",
    "\u{1f0c}",
    "\u{1f0e}",
    "\u{1f09}",
    "\u{1f0b}",
    "\u{1f0d}",
    "\u{1f0f}",
    "\u{1fbc}",
    "\u{1fef}\u{1fbc}",
    "\u{0384}\u{1fbc}",
    "\u{1fc0}\u{1fbc}",
    "\u{1f88}",
    "\u{1f8a}",
    "\u{1f8c}",
    "\u{1f8e}",
    "\u{1f89}",
    "\u{1f8b}",
    "\u{1f8d}",
    "\u{1f8f}",
];
const CAP_EPSILON: &[&str] = &[
    "\u{0395}", "\u{1fc8}", "\u{0388}", "\u{0395}", "\u{1f18}", "\u{1f1a}", "\u{1f1c}", "\u{1f18}",
    "\u{1f19}", "\u{1f1b}", "\u{1f1d}", "\u{1f19}",
];
const CAP_ETA: &[&str] = &[
    "\u{0397}",
    "\u{1fca}",
    "\u{0389}",
    "\u{1fc0}\u{0397}",
    "\u{1f28}",
    "\u{1f2a}",
    "\u{1f2c}",
    "\u{1f2e}",
    "\u{1f29}",
    "\u{1f2b}",
    "\u{1f2d}",
    "\u{1f2f}",
    "\u{1fcc}",
    "\u{1fef}\u{1fcc}",
    "\u{0384}\u{1fcc}",
    "\u{1fc0}\u{1fcc}",
    "\u{1f98}",
    "\u{1f9a}",
    "\u{1f9c}",
    "\u{1f9e}",
    "\u{1f99}",
    "\u{1f9b}",
    "\u{1f9d}",
    "\u{1f9f}",
];
const CAP_IOTA: &[&str] = &[
    "\u{0399}",
    "\u{1fda}",
    "\u{038a}",
    "\u{1fc0}\u{0399}",
    "\u{1f38}",
    "\u{1f3a}",
    "\u{1f3c}",
    "\u{1f3e}",
    "\u{1f39}",
    "\u{1f3b}",
    "\u{1f3d}",
    "\u{1f3f}",
    "\u{03aa}",
    "\u{1fef}\u{03aa}",
    "\u{0384}\u{03aa}",
    "\u{1fc0}\u{03aa}",
    "\u{03aa}\u{1fbd}",
    "\u{1fef}\u{03aa}\u{1fbd}",
    "\u{0384}\u{03aa}\u{1fbd}",
    "\u{1fc0}\u{03aa}\u{1fbd}",
    "\u{1ffe}\u{03aa}",
    "\u{1fdd}\u{03aa}",
    "\u{1fde}\u{03aa}",
    "\u{1fdf}\u{03aa}",
];
const CAP_OMICRON: &[&str] = &[
    "\u{039f}", "\u{1ff8}", "\u{038c}", "\u{039f}", "\u{1f48}", "\u{1f4a}", "\u{1f4c}", "\u{1f48}",
    "\u{1f49}", "\u{1f4b}", "\u{1f4d}", "\u{1f49}",
];
const CAP_YPSILON: &[&str] = &[
    "\u{03a5}",
    "\u{1fea}",
    "\u{038e}",
    "\u{1fc0}\u{03a5}",
    "\u{1fbf}\u{03a5}",
    "\u{1fcd}\u{03a5}",
    "\u{1fce}\u{03a5}",
    "\u{1fcf}\u{03a5}",
    "\u{1f59}",
    "\u{1f5b}",
    "\u{1f5d}",
    "\u{1f5f}",
    "\u{03ab}",
    "\u{1fef}\u{03ab}",
    "\u{0384}\u{03ab}",
    "\u{1fc0}\u{03ab}",
    "\u{03ab}\u{1fbd}",
    "\u{1fef}\u{03ab}\u{1fbd}",
    "\u{0384}\u{03ab}\u{1fbd}",
    "\u{1fc0}\u{03ab}\u{1fbd}",
    "\u{1ffe}\u{03ab}",
    "\u{1fdd}\u{03ab}",
    "\u{1fde}\u{03ab}",
    "\u{1fdf}\u{03ab}",
];
const CAP_OMEGA: &[&str] = &[
    "\u{03a9}",
    "\u{1ffa}",
    "\u{038f}",
    "\u{1fc0}\u{03a9}",
    "\u{1f68}",
    "\u{1f6a}",
    "\u{1f6c}",
    "\u{1f6e}",
    "\u{1f69}",
    "\u{1f6b}",
    "\u{1f6d}",
    "\u{1f6f}",
    "\u{1ffc}",
    "\u{1fef}\u{1ffc}",
    "\u{0384}\u{1ffc}",
    "\u{1fc0}\u{1ffc}",
    "\u{1fa8}",
    "\u{1faa}",
    "\u{1fac}",
    "\u{1fae}",
    "\u{1fa9}",
    "\u{1fab}",
    "\u{1fad}",
    "\u{1faf}",
];
