use super::*;

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Filter<I, P> {
    iter: I,
    f: P,
}

impl<I, F> Filter<I, F> {
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<I, F, R> TryIterator for Filter<I, F>
where
    I: TryIterator,
    F: FnMut(&I::Item) -> R,
    R: Try<Ok = bool>,
    R::Error: From<I::Error>,
{
    type Item = I::Item;
    type Error = R::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint().without_lower_bound()
    }

    fn try_fold<Acc, G, Q>(&mut self, acc: Acc, mut g: G) -> Q
    where
        G: FnMut(Acc, Self::Item) -> Q,
        Q: Try<Ok = Acc>,
        Q::Error: From<Self::Error>,
    {
        let f = &mut self.f;
        self.iter.map_err_mut(From::from).try_fold(acc, |acc, x| {
            if f(&x)? {
                g(acc, x)
            } else {
                Try::from_ok(acc)
            }
        })
    }
}

impl<I, F, R> DoubleEndedTryIterator for Filter<I, F>
where
    I: DoubleEndedTryIterator,
    F: FnMut(&I::Item) -> R,
    R: Try<Ok = bool>,
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
        self.iter.map_err_mut(From::from).try_rfold(acc, |acc, x| {
            if f(&x)? {
                g(acc, x)
            } else {
                Try::from_ok(acc)
            }
        })
    }
}

impl<I, F, R> FusedTryIterator for Filter<I, F>
where
    I: FusedTryIterator,
    F: FnMut(&I::Item) -> R,
    R: Try<Ok = bool>,
    R::Error: From<I::Error>,
{
}
