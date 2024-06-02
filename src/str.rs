//! Contains extentions for rust `str`.

use crate::span::Span;

pub trait StrExt {
    /// Gets the position of a substring within a string.
    /// # Example
    /// ```rust
    /// # use wutil::prelude::*;
    /// # use wutil::span::Span;
    /// let place = "mississippis";
    ///
    /// let second_is: &str = place.matches("is").nth(1).unwrap();
    ///
    /// assert_eq!(place.substr_pos(second_is), Some(Span::from(4..6)));
    /// ```
    fn substr_pos(&self, substr: &Self) -> Option<Span>;
    /// Gets the length of the character starting at `byte_index`
    fn char_length(&self, byte_index: usize) -> Option<usize>;
    /// Gets the range of the character starting at `byte_index`
    fn char_span(&self, byte_index: usize) -> Option<Span>;
}

impl StrExt for str {
    fn substr_pos(&self, substr: &Self) -> Option<Span> {
        let self_start = self.as_ptr() as usize;
        let substr_start = substr.as_ptr() as usize;

        let start = substr_start.wrapping_sub(self_start);
        let end = start + substr.len();

        if start > self.len() || end > self.len() {
            return None;
        }

        Some(Span::from(start..end))
    }

    fn char_length(&self, byte_index: usize) -> Option<usize> {
        let subsl = self.get(byte_index..)?;
        let mut iter = subsl.char_indices();
        iter.next()?;

        Some(iter.next().map(|s| s.0).unwrap_or(subsl.len()))
    }

    fn char_span(&self, byte_index: usize) -> Option<Span> {
        Some(Span::at(byte_index).with_len(self.char_length(byte_index)?))
    }
}
