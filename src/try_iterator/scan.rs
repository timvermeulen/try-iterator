use super::*;

pub struct Scan<I, St, F> {
    iter: I,
    state: St,
    f: F,
}

impl<I, St, F, R, T> Scan<I, St, F>
where
    I: TryIterator,
    F: FnMut(&mut St, I::Item) -> R,
    R: Try<Ok = Option<T>>,
    R::Error: From<I::Error>,
{
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
                None => LoopState::BreakValue(acc),
                Some(x) => LoopState::continue_with_try(g(acc, x)),
            })
            .into_try()
    }
}
