use super::*;

pub struct Flatten<I, U> {
    iter: I,
    front: Option<U>,
    back: Option<U>,
}

impl<I, U, E> Flatten<I, U>
where
    I: TryIterator<Error = E>,
    U: TryIterator,
    I::Item: IntoTryIterator<Item = U::Item, Error = U::Error, IntoTryIter = U>,
    E: From<U::Error>,
{
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            front: None,
            back: None,
        }
    }

    fn iter_try_fold<Acc, Fold, R>(&mut self, mut acc: Acc, mut fold: Fold) -> R
    where
        Fold: FnMut(Acc, &mut U) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<E>,
    {
        let mut fold = |acc, iter: &mut _| -> R {
            let acc = match iter {
                None => acc,
                Some(ref mut iter) => fold(acc, iter)?,
            };
            *iter = None;
            Try::from_ok(acc)
        };

        acc = fold(acc, &mut self.front)?;

        let front = &mut self.front;
        acc = self.iter.try_fold(acc, |acc, iter| {
            *front = Some(iter.into_try_iter());
            fold(acc, front)
        })?;

        fold(acc, &mut self.back)
    }
}

impl<I, U, E> TryIterator for Flatten<I, U>
where
    I: TryIterator<Error = E>,
    U: TryIterator,
    I::Item: IntoTryIterator<Item = U::Item, Error = U::Error, IntoTryIter = U>,
    E: From<U::Error>,
{
    type Item = U::Item;
    type Error = E;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn try_fold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        self.iter_try_fold(acc, move |acc, iter| {
            iter.map_err_mut(From::from).try_fold(acc, &mut f)
        })
    }
}
