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

    fn try_fold<Acc, G, R>(&mut self, acc: Acc, mut g: G) -> R
    where
        Self: Sized,
        G: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        match self.iter.try_fold(acc, |acc, x| {
            g(acc, x).into_result().map_err(MappedErr::Map)
        }) {
            Ok(x) => Try::from_ok(x),
            Err(MappedErr::Iter(e)) => Try::from_error((self.f)(e).into()),
            Err(MappedErr::Map(e)) => Try::from_error(e),
        }
    }
}

enum MappedErr<I, M> {
    Iter(I),
    Map(M),
}

impl<I, M> From<I> for MappedErr<I, M> {
    fn from(x: I) -> MappedErr<I, M> {
        MappedErr::Iter(x)
    }
}
