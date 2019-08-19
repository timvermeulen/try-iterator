use super::*;

pub struct IntoResults<I> {
    iter: I,
}

impl<I> IntoResults<I>
where
    I: TryIterator,
{
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I> Iterator for IntoResults<I>
where
    I: TryIterator,
{
    type Item = Result<I::Item, I::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().transpose()
    }
}
