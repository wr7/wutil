//! Contains extensions for rust iterators.

use core::{mem::MaybeUninit, ptr::addr_of};

pub trait IterExt {
    type Item;

    /// Gets `N` items from an iterator and returns them as an array. Otherwise returns `None`.
    /// # Example
    /// ```rust
    /// # use wutil::prelude::*;
    /// let mut tens_iterator = (0..=100).filter(|n| n % 10 == 0);
    /// let nums: [u32; 5] = tens_iterator.collect_n::<5>().unwrap();
    ///
    /// assert_eq!(&nums, &[0, 10, 20, 30, 40]);
    /// ```
    fn collect_n<const N: usize>(&mut self) -> Option<[Self::Item; N]>;
}

impl<I: Iterator> IterExt for I {
    type Item = I::Item;

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
