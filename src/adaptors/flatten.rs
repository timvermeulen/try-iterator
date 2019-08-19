use super::*;

pub struct Flatten<I, U> {
    iter: I,
    front: Option<U>,
    back: Option<U>,
}

impl<I, U> Flatten<I, U>
where
    I: TryIterator,
    U: TryIterator,
    I::Item: IntoTryIterator<Item = U::Item, Error = U::Error, IntoTryIter = U>,
    I::Error: From<U::Error>,
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
        R::Error: From<I::Error>,
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

impl<I, U> TryIterator for Flatten<I, U>
where
    I: TryIterator,
    U: TryIterator,
    I::Item: IntoTryIterator<Item = U::Item, Error = U::Error, IntoTryIter = U>,
    I::Error: From<U::Error>,
{
    type Item = U::Item;
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size_hint = |iter: &Option<U>| match iter {
            None => size_hint::ZERO,
            Some(ref iter) => iter.size_hint(),
        };
        let (front_lower, front_upper) = size_hint(&self.front);
        let (back_lower, back_upper) = size_hint(&self.back);

        let lower = front_lower.saturating_add(back_lower);
        let upper = match (self.iter.size_hint(), front_upper, back_upper) {
            (size_hint::ZERO, Some(x), Some(y)) => x.checked_add(y),
            _ => None,
        };

        (lower, upper)
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
