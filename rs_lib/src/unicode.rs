// [[no accent, grave, acute, circumflex](no breathing, smooth, rough)](no subscript, iota subscript)
// [[no accent, grave, acute, circumflex](no breathing, smooth, rough)](no diaeresis, diaeresis)
mod vowel;
pub use vowel::*;

mod consonant;
pub use consonant::*;

pub const KORONIS: &str = "\u{1fbd}";
