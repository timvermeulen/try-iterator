use super::*;

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Scan<I, St, F> {
    iter: I,
    state: St,
    f: F,
}

impl<I, St, F> Scan<I, St, F> {
    pub(crate) fn new(iter: I, state: St, f: F) -> Self {
        Self { iter, f, state }
    }
}

impl<I, St, F, R, T> TryIterator for Scan<I, St, F>
where
    I: TryIterator,
    F: FnMut(&mut St, I::Item) -> R,
    R: Try<Ok = Option<T>>,
    R::Error: From<I::Error>,
{
    type Item = T;
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
        let state = &mut self.state;
        let f = &mut self.f;
        self.iter
            .map_err_mut(Self::Error::from)
            .try_fold(acc, move |acc, x| match f(state, x)? {
                None => LoopState::Break(acc),
                Some(x) => LoopState::continue_with_try(g(acc, x)),
            })
            .into_try()
    }
}
