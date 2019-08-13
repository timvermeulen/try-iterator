use super::*;

pub struct Filter<I, P> {
    iter: I,
    predicate: P,
}

impl<I, P> Filter<I, P> {
    pub(crate) fn new<R>(iter: I, predicate: P) -> Self
    where
        I: TryIterator,
        P: FnMut(&I::Item) -> R,
        R: Try<Ok = bool>,
        R::Error: From<I::Error>,
    {
        Self { iter, predicate }
    }
}

impl<I, P, R> TryIterator for Filter<I, P>
where
    I: TryIterator,
    P: FnMut(&I::Item) -> R,
    R: Try<Ok = bool>,
    R::Error: From<I::Error>,
{
    type Item = I::Item;
    type Error = R::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn try_fold<Acc, F, Q>(&mut self, acc: Acc, mut f: F) -> Q
    where
        F: FnMut(Acc, Self::Item) -> Q,
        Q: Try<Ok = Acc>,
        Q::Error: From<Self::Error>,
    {
        let p = &mut self.predicate;
        self.iter.map_err_mut(From::from).try_fold(acc, |acc, x| {
            if p(&x)? {
                f(acc, x)
            } else {
                Try::from_ok(acc)
            }
        })
    }
}
