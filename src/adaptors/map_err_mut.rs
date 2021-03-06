use super::*;

#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct MapErrMut<'a, I, F> {
    iter: &'a mut I,
    f: F,
}

impl<'a, I, F> MapErrMut<'a, I, F> {
    pub(crate) fn new(iter: &'a mut I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<I, F, E> TryIterator for MapErrMut<'_, I, F>
where
    I: TryIterator,
    F: FnMut(I::Error) -> E,
{
    type Item = I::Item;
    type Error = E;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn try_fold<Acc, G, R>(&mut self, acc: Acc, mut g: G) -> R
    where
        Self: Sized,
        G: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        self.iter
            .try_fold(acc, |acc, x| LoopState::continue_with_try(g(acc, x)))
            .map_iter_error(&mut self.f)
            .map_break(|x: !| x)
            .into_try()
    }

    fn count(self) -> Result<usize, Self::Error> {
        self.iter.count().map_err(self.f)
    }

    fn last(self) -> Result<Option<Self::Item>, Self::Error> {
        self.iter.last().map_err(self.f)
    }
}

impl<I, F, E> DoubleEndedTryIterator for MapErrMut<'_, I, F>
where
    I: DoubleEndedTryIterator,
    F: FnMut(I::Error) -> E,
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.rfind(|_| true)
    }

    fn try_rfold<Acc, G, R>(&mut self, acc: Acc, mut g: G) -> R
    where
        Self: Sized,
        G: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        self.iter
            .try_rfold(acc, |acc, x| LoopState::continue_with_try(g(acc, x)))
            .map_iter_error(&mut self.f)
            .map_break(|x: !| x)
            .into_try()
    }
}

impl<I, F, E> ExactSizeTryIterator for MapErrMut<'_, I, F>
where
    I: ExactSizeTryIterator,
    F: FnMut(I::Error) -> E,
{
}

impl<I, F, E> FusedTryIterator for MapErrMut<'_, I, F>
where
    I: FusedTryIterator,
    F: FnMut(I::Error) -> E,
{
}
