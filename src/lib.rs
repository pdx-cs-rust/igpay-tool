//! StUdLy PiG library

mod makepig;

/// Produce pig latin studly text.
///
/// # Examples:
///
/// ```
/// # use studly_pig::*;
/// assert_eq!("uStDaY", studly_pig("dust"));
/// ```
pub fn studly_pig(text: &str) -> String {
    let pig = makepig::make_pig();
    let pigged = pig.text_to_pig_latin(text);
    let mut result = String::new();
    let mut odd = false;
    for ch in pigged.chars() {
        if odd {
            let uch = ch.to_uppercase();
            result.extend(uch);
        } else {
            result.push(ch);
        }
        odd = !odd;
    }
    result
}
