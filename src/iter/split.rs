/// Iterator returned by [`Iterator::split`](crate::prelude::IterCloneExt::split)
pub struct Split<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    iter: I,
    pred: F,
    consumed: bool,
}

impl<I, F> Split<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    pub(super) fn new(iter: I, pred: F) -> Self {
        Self {
            iter,
            pred,
            consumed: false,
        }
    }
}

/// Iterator used by [`Iterator::split`](crate::prelude::IterCloneExt::split)
pub struct SplitIterator<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    iter: I,
    pred: F,
    consumed: bool,
}

impl<I, F> Iterator for SplitIterator<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed {
            return None;
        }

        let item = self.iter.next()?;

        if (self.pred)(&item) {
            self.consumed = true;
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
        if self.consumed {
            return None;
        }

        let next_iter = self.iter.clone();

        loop {
            let Some(item) = self.iter.next() else {
                self.consumed = true;
                return Some(SplitIterator {
                    iter: next_iter,
                    pred: self.pred.clone(),
                    consumed: false,
                });
            };

            if (self.pred)(&item) {
                return Some(SplitIterator {
                    iter: next_iter,
                    pred: self.pred.clone(),
                    consumed: false,
                });
            }
        }
    }
}

/// Iterator returned by [`Iterator::split_inclusive`](crate::prelude::IterCloneExt::split_inclusive)
pub struct SplitInclusive<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    iter: I,
    pred: F,
    consumed: bool,
}

impl<I, F> SplitInclusive<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    pub(super) fn new(iter: I, pred: F) -> Self {
        Self {
            iter,
            pred,
            consumed: false,
        }
    }
}

/// Iterator used by [`Iterator::split_inclusive`](crate::prelude::IterCloneExt::split_inclusive)
pub struct InclusiveSplitIterator<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    iter: I,
    pred: F,
    consumed: bool,
}

impl<I, F> Iterator for InclusiveSplitIterator<I, F>
where
    I: Iterator + Clone,
    F: FnMut(&I::Item) -> bool + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed {
            return None;
        }

        let item = self.iter.next()?;

        if (self.pred)(&item) {
            self.consumed = true;
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
        if self.consumed {
            return None;
        }

        let next_iter = self.iter.clone();

        loop {
            let Some(item) = self.iter.next() else {
                self.consumed = true;
                return Some(InclusiveSplitIterator {
                    iter: next_iter,
                    pred: self.pred.clone(),
                    consumed: false,
                });
            };

            if (self.pred)(&item) {
                return Some(InclusiveSplitIterator {
                    iter: next_iter,
                    pred: self.pred.clone(),
                    consumed: false,
                });
            }
        }
    }
}
