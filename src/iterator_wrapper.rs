use super::*;

pub struct IteratorWrapper<I, E> {
    iter: I,
    _marker: PhantomData<E>,
}

impl<I, E> IteratorWrapper<I, E>
where
    I: Iterator,
{
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            _marker: PhantomData,
        }
    }
}

impl<I, E> TryIterator for IteratorWrapper<I, E>
where
    I: Iterator,
{
    type Item = I::Item;
    type Error = E;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        Ok(self.iter.next())
    }
}
