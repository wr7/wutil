/// Iterator returned by [`SliceExt.with_sequence_removed(..)`].
///
/// [`SliceExt.with_sequence_removed(..)`]: super::SliceExt::with_sequence_removed
pub struct WithSequenceRemoved<'a, T>
where
    T: PartialEq,
{
    sequence: &'a [T],
    remaining: &'a [T],
}

impl<'a, T> WithSequenceRemoved<'a, T>
where
    T: PartialEq,
{
    pub(super) fn new(slice: &'a [T], sequence: &'a [T]) -> Self {
        Self {
            remaining: slice,
            sequence,
        }
    }
}

impl<'a, T> Iterator for WithSequenceRemoved<'a, T>
where
    T: PartialEq,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.remaining.starts_with(self.sequence) {
                self.remaining = &self.remaining.get(self.sequence.len()..).unwrap_or(&[]);
                continue;
            }

            let (first_element, remaining) = self.remaining.split_first()?;
            self.remaining = remaining;

            return Some(first_element);
        }
    }
}
