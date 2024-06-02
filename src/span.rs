//! Contains a thin `Copy` wrapper around `Range<usize>`. This has the same layout as `Range<usize>`.

use core::{
    borrow::{Borrow, BorrowMut},
    mem,
    ops::{Deref, DerefMut, Index, IndexMut, Range},
};

pub use crate::Span;

// So that it `Span` appears defined at root-level
pub(crate) mod s {
    use core::{
        mem::{self, MaybeUninit},
        ops::Range,
    };

    /// A thin `Copy` wrapper around `Range<usize>`. This has the same layout as `Range<usize>`.
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct Span {
        inner: MaybeUninit<[u8; mem::size_of::<Range<usize>>()]>,
    }
}

impl Span {
    /// A zero-width span at the end of a span
    pub fn span_after(self) -> Self {
        (self.end..self.end).into()
    }

    /// A zero-width span at the start of a span
    pub fn span_at(self) -> Self {
        (self.start..self.start).into()
    }

    /// Sets the length of a span without changing its start
    pub fn with_len(self, len: usize) -> Self {
        (self.start..self.start + len).into()
    }

    /// Sets the start of a span without changing its end
    pub fn with_start(self, start: usize) -> Self {
        (start..self.end).into()
    }

    /// Sets the end of a span without changing its start
    pub fn with_end(self, end: usize) -> Self {
        (self.start..end).into()
    }

    /// Returns the zero-width span at a certain position
    /// ```rust
    /// # use wutil::span::Span;
    /// assert_eq!(Span::at(5), Span::from(5..5));
    /// assert_eq!(Span::at(0), Span::from(0..0));
    /// assert_eq!(Span::at(50), Span::from(50..50));
    /// ```
    pub fn at(pos: usize) -> Self {
        (pos..pos).into()
    }

    /// Checks if this `Span` overlaps with another `Span`.
    /// ```rust
    /// # use wutil::span::Span;
    /// let foo = Span::from(0..3);
    /// let bar = Span::from(2..4);
    /// let biz = Span::from(3..6);
    ///
    /// assert!(!foo.overlaps_with(biz));
    /// assert!(foo.overlaps_with(bar));
    /// assert!(bar.overlaps_with(biz));
    ///
    /// assert!(foo.overlaps_with(Span::from(0..0)));
    /// ```
    pub fn overlaps_with(self, other: Self) -> bool {
        self.contains(&other.start) || other.contains(&self.start)
    }
}

/// An iterator for `Span::into_iter`. This is here in-case `Range<usize>` stops being an iterator
pub struct SpanIterator {
    inner: Range<usize>,
}

impl Iterator for SpanIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[cfg(feature = "alloc")]
mod debug {
    use core::{fmt::Debug, ops::Deref};

    use super::Span;

    impl Debug for Span {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            Debug::fmt(self.deref(), f)
        }
    }
}

// Boilerplate //

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other)
    }
}

impl Eq for Span {}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        unsafe { mem::transmute::<Range<usize>, Span>(value) }
    }
}

impl From<Span> for Range<usize> {
    fn from(value: Span) -> Self {
        unsafe { mem::transmute::<Span, Range<usize>>(value) }
    }
}

impl AsRef<Range<usize>> for Span {
    fn as_ref(&self) -> &Range<usize> {
        unsafe { crate::transmute_ref::<Span, Range<usize>>(self) }
    }
}

impl AsRef<Span> for Range<usize> {
    fn as_ref(&self) -> &Span {
        unsafe { crate::transmute_ref::<Range<usize>, Span>(self) }
    }
}

impl AsMut<Range<usize>> for Span {
    fn as_mut(&mut self) -> &mut Range<usize> {
        unsafe { crate::transmute_mut::<Span, Range<usize>>(self) }
    }
}

impl AsMut<Span> for Range<usize> {
    fn as_mut(&mut self) -> &mut Span {
        unsafe { crate::transmute_mut::<Range<usize>, Span>(self) }
    }
}

impl Deref for Span {
    type Target = Range<usize>;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl DerefMut for Span {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl Borrow<Range<usize>> for Span {
    fn borrow(&self) -> &Range<usize> {
        self.as_ref()
    }
}

impl BorrowMut<Range<usize>> for Span {
    fn borrow_mut(&mut self) -> &mut Range<usize> {
        self.as_mut()
    }
}

impl<T> Index<Span> for [T] {
    type Output = [T];

    fn index(&self, index: Span) -> &Self::Output {
        &self[index.deref().clone()]
    }
}

impl<T> IndexMut<Span> for [T] {
    fn index_mut(&mut self, index: Span) -> &mut Self::Output {
        &mut self[index.deref().clone()]
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[index.deref().clone()]
    }
}

impl IndexMut<Span> for str {
    fn index_mut(&mut self, index: Span) -> &mut Self::Output {
        &mut self[index.deref().clone()]
    }
}

impl IntoIterator for Span {
    type Item = usize;

    type IntoIter = SpanIterator;

    fn into_iter(self) -> Self::IntoIter {
        SpanIterator { inner: self.into() }
    }
}
