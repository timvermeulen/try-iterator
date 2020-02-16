use super::*;

pub trait ExactSizeTryIterator: TryIterator {
    fn len(&self) -> usize {
        let (lower, upper) = self.size_hint();
        assert_eq!(Some(lower), upper);
        lower
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<I> ExactSizeTryIterator for &mut I
where I: ExactSizeTryIterator + ?Sized
{
    fn len(&self) -> usize {
        (**self).len()
    }

    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }
}
