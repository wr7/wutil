//! Contains extentions for rust slices.

mod with_sequence_removed;
pub use with_sequence_removed::WithSequenceRemoved;

impl<T> SliceExt for [T] {
    type Inner = T;

    fn find(&self, subslice: &[T]) -> Option<usize>
    where
        T: PartialEq,
    {
        self.windows(subslice.len()).position(|w| w == subslice)
    }

    fn with_sequence_removed<'a>(&'a self, sequence: &'a [T]) -> WithSequenceRemoved<T>
    where
        T: PartialEq,
    {
        WithSequenceRemoved::new(self, sequence)
    }

    fn get_slice_between(&self, slice1: &[T], slice2: &[T]) -> Option<&[T]>
    where
        T: PartialEq,
    {
        let start_idx = self.find(slice1)? + slice1.len();

        let slice = self.get(start_idx..)?;
        let end_idx = slice.find(slice2)?;

        slice.get(0..end_idx)
    }
}

pub trait SliceExt {
    type Inner;

    /// Returns the starting index of the first occurence of `subslice`.
    fn find(&self, subslice: &Self) -> Option<usize>
    where
        Self::Inner: PartialEq;

    /// Returns an iterator with the provided sequence filtered out once.
    /// # Example
    /// ```
    /// # use wutil::prelude::*;
    /// let Mssippi: Vec<u8> = b"Mississippi".with_sequence_removed(b"is").copied().collect();
    /// assert_eq!(&Mssippi, b"Mssippi");
    ///
    /// let ab: Vec<u8> = b"aabb".with_sequence_removed(b"ab").copied().collect();
    /// assert_eq!(&ab, b"ab");
    ///
    /// let a: Vec<u8> = b"aaaaa".with_sequence_removed(b"aa").copied().collect();
    /// assert_eq!(&a, b"a");
    /// ```
    fn with_sequence_removed<'a>(
        &'a self,
        sequence: &'a [Self::Inner],
    ) -> WithSequenceRemoved<Self::Inner>
    where
        Self::Inner: PartialEq;

    /// Gets the content between the first occurance of two subslices.
    /// # Example
    /// ```
    /// # use wutil::prelude::*;
    /// let text = b"<a>foo</a> bar 'biz'] [bax] <a>bang</a>";
    ///
    /// assert!(text.get_slice_between(b"<a>", b"</a>") == Some(b"foo"));
    /// assert!(text.get_slice_between(b"'", b"'") == Some(b"biz"));
    /// assert!(text.get_slice_between(b"[", b"]") == Some(b"bax"));
    /// ```
    fn get_slice_between(&self, slice1: &Self, slice2: &Self) -> Option<&Self>
    where
        Self::Inner: PartialEq;
}
