use super::*;

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct IntoResults<I> {
    iter: I,
}

impl<I> IntoResults<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I> Iterator for IntoResults<I>
where I: TryIterator
{
    type Item = Result<I::Item, I::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().transpose()
    }
}
