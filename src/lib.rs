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
    makepig::make_pig()
        .text_to_pig_latin(text)
        .chars()
        .enumerate()
        .flat_map(|(i, ch)| -> Box<dyn Iterator<Item = char>> {
            if i % 2 == 1 {
                Box::new(ch.to_uppercase())
            } else {
                Box::new(std::iter::once(ch))
            }
        })
        .collect()
}
