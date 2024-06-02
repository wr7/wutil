//! Contains extensions for rust iterators.

use core::{mem::MaybeUninit, ptr::addr_of};

mod split;

pub use split::*;

/// An extension trait for `Iterator`
pub trait IterExt: Iterator {
    /// Gets `N` items from an iterator and returns them as an array. Otherwise returns `None`.
    /// # Example
    /// ```rust
    /// # use wutil::prelude::*;
    /// let mut tens_iterator = (0..=100).filter(|n| n % 10 == 0);
    /// let nums: [u32; 5] = tens_iterator.collect_n::<5>().unwrap();
    ///
    /// assert_eq!(&nums, &[0, 10, 20, 30, 40]);
    /// ```
    fn collect_n<const N: usize>(&mut self) -> Option<[Self::Item; N]> {
        let mut arr: [MaybeUninit<Self::Item>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..N {
            let Some(item) = self.next() else {
                for item in &mut arr[0..i] {
                    unsafe { item.assume_init_drop() };
                }
                return None;
            };

            arr[i].write(item);
        }

        Some(unsafe { addr_of!(arr).cast::<[Self::Item; N]>().read() })
    }
}

/// An extension trait for `Iterator + Clone`
pub trait IterCloneExt: Iterator + Clone {
    /// Splits an iterator into an iterator of iterators.
    /// # Example
    /// ```rust
    /// # use wutil::prelude::*;
    /// let nums = [0u32, 10, 20, 0, 0, 5, 50, 0];
    ///
    /// let split_nums: Vec<Vec<u32>> = nums
    ///     .iter()
    ///     .filter(|n| *n % 2 == 0)
    ///     .split(|n| **n == 0)
    ///     .map(|n| n.copied().collect::<Vec<u32>>())
    ///     .collect();
    ///
    /// let expected: &[&[u32]] = &[&[], &[10, 20], &[], &[50], &[]];
    ///
    /// assert_eq!(split_nums, expected);
    /// ```
    fn split<P>(self, pred: P) -> Split<Self, P>
    where
        P: FnMut(&Self::Item) -> bool + Clone,
    {
        Split::new(self, pred)
    }
}

impl<I: Iterator> IterExt for I {}

impl<I> IterCloneExt for I where I: Iterator + Clone {}
