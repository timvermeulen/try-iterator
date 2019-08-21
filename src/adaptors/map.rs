use super::*;

pub struct Map<I, F> {
    iter: I,
    f: F,
}

impl<I, F> Map<I, F> {
    pub(crate) fn new<R>(iter: I, f: F) -> Self
    where
        I: TryIterator,
        F: FnMut(I::Item) -> R,
        R: Try,
        R::Error: From<I::Error>,
    {
        Self { iter, f }
    }
}

impl<I, F, R> TryIterator for Map<I, F>
where
    I: TryIterator,
    F: FnMut(I::Item) -> R,
    R: Try,
    R::Error: From<I::Error>,
{
    type Item = R::Ok;
    type Error = R::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn try_fold<Acc, G, Q>(&mut self, acc: Acc, mut g: G) -> Q
    where
        G: FnMut(Acc, Self::Item) -> Q,
        Q: Try<Ok = Acc>,
        Q::Error: From<Self::Error>,
    {
        let f = &mut self.f;
        self.iter
            .map_err_mut(From::from)
            .try_fold(acc, |acc, x| g(acc, f(x)?))
    }
}

impl<I, F, R> DoubleEndedTryIterator for Map<I, F>
where
    I: DoubleEndedTryIterator,
    F: FnMut(I::Item) -> R,
    R: Try,
    R::Error: From<I::Error>,
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.rfind(|_| true)
    }

    fn try_rfold<Acc, G, Q>(&mut self, acc: Acc, mut g: G) -> Q
    where
        G: FnMut(Acc, Self::Item) -> Q,
        Q: Try<Ok = Acc>,
        Q::Error: From<Self::Error>,
    {
        let f = &mut self.f;
        self.iter
            .map_err_mut(From::from)
            .try_rfold(acc, |acc, x| g(acc, f(x)?))
    }
}

impl<I, F, R> ExactSizeTryIterator for Map<I, F>
where
    I: ExactSizeTryIterator,
    F: FnMut(I::Item) -> R,
    R: Try,
    R::Error: From<I::Error>,
{
}

impl<I, F, R> FusedTryIterator for Map<I, F>
where
    I: FusedTryIterator,
    F: FnMut(I::Item) -> R,
    R: Try,
    R::Error: From<I::Error>,
{
}
