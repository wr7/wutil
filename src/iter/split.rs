/// Iterator returned by [`Iterator::split`](crate::prelude::IterCloneExt::split)
pub struct Split<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    iter: Option<I>,
    pred: F,
}

impl<I, F> Split<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    pub(super) fn new(iter: I, pred: F) -> Self {
        Self {
            iter: Some(iter),
            pred,
        }
    }
}

/// Iterator used by [`Iterator::split`](crate::prelude::IterCloneExt::split)
pub struct SplitIterator<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    iter: Option<I>,
    pred: F,
}

impl<I, F> Iterator for SplitIterator<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.as_mut()?.next()?;

        if (self.pred)(&item) {
            self.iter = None;
            return None;
        }

        Some(item)
    }
}

impl<I, F> Iterator for Split<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    type Item = SplitIterator<I, F>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_iter = self.iter.clone()?;

        loop {
            let Some(item) = self.iter.as_mut()?.next() else {
                self.iter = None;
                break;
            };

            if (self.pred)(&item) {
                break;
            }
        }

        Some(SplitIterator {
            iter: Some(next_iter),
            pred: self.pred.clone(),
        })
    }
}

/// Iterator returned by [`Iterator::split_inclusive`](crate::prelude::IterCloneExt::split_inclusive)
pub struct SplitInclusive<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    iter: Option<I>,
    pred: F,
}

impl<I, F> SplitInclusive<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    pub(super) fn new(iter: I, pred: F) -> Self {
        Self {
            iter: Some(iter),
            pred,
        }
    }
}

/// Iterator used by [`Iterator::split_inclusive`](crate::prelude::IterCloneExt::split_inclusive)
pub struct InclusiveSplitIterator<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    iter: Option<I>,
    pred: F,
}

impl<I, F> Iterator for InclusiveSplitIterator<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.as_mut()?.next()?;

        if (self.pred)(&item) {
            self.iter = None;
        }

        Some(item)
    }
}

impl<I, F> Iterator for SplitInclusive<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    type Item = InclusiveSplitIterator<I, F>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_iter = self.iter.clone()?;

        loop {
            let Some(item) = self.iter.as_mut()?.next() else {
                self.iter = None;
                break;
            };

            if (self.pred)(&item) {
                break;
            }
        }

        Some(InclusiveSplitIterator {
            iter: Some(next_iter),
            pred: self.pred.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use alloc::vec::Vec;

    #[test]
    fn split() {
        let tests: &[(&[u32], &[&[u32]])] = &[
            (
                &[0, 10, 20, 0, 0, 50, 0],
                &[&[], &[10, 20], &[], &[50], &[]],
            ),
            (
                &[0, 10, 20, 0, 0, 50, 0, 2],
                &[&[], &[10, 20], &[], &[50], &[2]],
            ),
        ];

        for (input, expected) in tests {
            let split_nums: Vec<Vec<u32>> = input
                .iter()
                .split(|n| **n == 0)
                .map(|n| n.copied().collect::<Vec<u32>>())
                .collect();

            assert_eq!(&split_nums, expected);
        }
    }

    #[test]
    fn split_inclusive() {
        let tests: &[(&[u32], &[&[u32]])] = &[
            (
                &[0, 10, 20, 0, 0, 50, 0],
                &[&[0], &[10, 20, 0], &[0], &[50, 0], &[]],
            ),
            (
                &[0, 10, 20, 0, 0, 50, 0, 2],
                &[&[0], &[10, 20, 0], &[0], &[50, 0], &[2]],
            ),
        ];

        for (input, expected_output) in tests {
            let split_nums: Vec<Vec<u32>> = input
                .iter()
                .split_inclusive(|n| **n == 0)
                .map(|n| n.copied().collect::<Vec<u32>>())
                .collect();

            assert_eq!(&split_nums, expected_output);
        }
    }
}
