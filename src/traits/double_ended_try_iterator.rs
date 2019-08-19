use super::*;

pub trait DoubleEndedTryIterator: TryIterator {
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error>;

    fn rev(self) -> Rev<Self>
    where
        Self: Sized,
    {
        Rev::new(self)
    }
}

impl<I> DoubleEndedTryIterator for &mut I
where
    I: DoubleEndedTryIterator + ?Sized,
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        (**self).next_back()
    }
}
