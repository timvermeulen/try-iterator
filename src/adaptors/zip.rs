use super::*;

pub struct Zip<A, B> {
    a: A,
    b: B,
}

impl<A, B> Zip<A, B>
where
    A: TryIterator,
    B: TryIterator,
    A::Error: From<B::Error>,
{
    pub(crate) fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A, B> TryIterator for Zip<A, B>
where
    A: TryIterator,
    B: TryIterator,
    A::Error: From<B::Error>,
{
    type Item = (A::Item, B::Item);
    type Error = A::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        let (x, y) = (self.a.next()?, self.b.next()?);
        Ok(try { (x?, y?) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        size_hint::min(self.a.size_hint(), self.b.size_hint())
    }

    fn try_fold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        let b = &mut self.b;
        self.a
            .try_fold(acc, |acc, x| {
                match b.map_err_mut(A::Error::from).next()? {
                    None => LoopState::Break(acc),
                    Some(y) => LoopState::continue_with_try(f(acc, (x, y))),
                }
            })
            .into_try()
    }
}

impl<A, B> ExactSizeTryIterator for Zip<A, B>
where
    A: ExactSizeTryIterator,
    B: ExactSizeTryIterator,
    A::Error: From<B::Error>,
{
}

impl<A, B> FusedTryIterator for Zip<A, B>
where
    A: FusedTryIterator,
    B: FusedTryIterator,
    A::Error: From<B::Error>,
{
}
