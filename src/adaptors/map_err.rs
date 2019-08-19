use super::*;

pub struct MapErr<I, F> {
    iter: I,
    f: F,
}

impl<I, F> MapErr<I, F> {
    pub(crate) fn new<E>(iter: I, f: F) -> Self
    where
        I: TryIterator,
        F: FnMut(I::Error) -> E,
    {
        Self { iter, f }
    }
}

impl<I, F, E> TryIterator for MapErr<I, F>
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
            .map_break_value(|x: !| x)
            .into_try()
    }
}

impl<I, F, E> ExactSizeTryIterator for MapErr<I, F>
where
    I: ExactSizeTryIterator,
    F: FnMut(I::Error) -> E,
{
}
