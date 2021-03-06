use super::*;

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Flatten<I, U> {
    iter: I,
    front: Option<U>,
    back: Option<U>,
}

impl<I, U> Flatten<I, U> {
    pub(crate) fn new(iter: I) -> Self {
        Self { iter, front: None, back: None }
    }
}

impl<I, U> Flatten<I, U>
where
    I: TryIterator,
    U: TryIterator,
    I::Item: IntoTryIterator<Item = U::Item, Error = U::Error, IntoTryIter = U>,
    I::Error: From<U::Error>,
{
    fn iter_try_fold<Acc, F, R>(&mut self, mut acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, &mut U) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<I::Error>,
    {
        let mut fold = |acc, iter: &mut _| -> R {
            let acc = match iter {
                None => acc,
                Some(iter) => f(acc, iter)?,
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

impl<I, U> Flatten<I, U>
where
    I: DoubleEndedTryIterator,
    U: DoubleEndedTryIterator,
    I::Item: IntoTryIterator<Item = U::Item, Error = U::Error, IntoTryIter = U>,
    I::Error: From<U::Error>,
{
    fn iter_try_rfold<Acc, F, R>(&mut self, mut acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, &mut U) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<I::Error>,
    {
        let mut fold = |acc, iter: &mut _| -> R {
            let acc = match iter {
                None => acc,
                Some(iter) => f(acc, iter)?,
            };
            *iter = None;
            Try::from_ok(acc)
        };

        acc = fold(acc, &mut self.front)?;

        let front = &mut self.front;
        acc = self.iter.try_rfold(acc, |acc, iter| {
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

    fn try_nth(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        self.iter_try_fold(n, |n, iter| match iter.map_err_mut(I::Error::from).try_nth(n)? {
            Ok(x) => LoopState::Break(x),
            Err(n) => LoopState::Continue(n),
        })
        .map_continue(Err)
        .map_break(Ok)
        .into_try()
    }

    fn try_fold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        self.iter_try_fold(acc, move |acc, iter| iter.map_err_mut(From::from).try_fold(acc, &mut f))
    }

    fn count(mut self) -> Result<usize, Self::Error> {
        self.iter_try_fold(0, |acc, iter| Ok(acc + iter.count()?))
    }

    fn last(mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.iter_try_fold(None, |last, iter| Ok(iter.last()?.or(last)))
    }
}

impl<I, U> DoubleEndedTryIterator for Flatten<I, U>
where
    I: DoubleEndedTryIterator,
    U: DoubleEndedTryIterator,
    I::Item: IntoTryIterator<Item = U::Item, Error = U::Error, IntoTryIter = U>,
    I::Error: From<U::Error>,
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.rfind(|_| true)
    }

    fn try_nth_back(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        self.iter_try_rfold(n, |n, iter| {
            match iter.map_err_mut(I::Error::from).try_nth_back(n)? {
                Ok(x) => LoopState::Break(x),
                Err(n) => LoopState::Continue(n),
            }
        })
        .map_continue(Err)
        .map_break(Ok)
        .into_try()
    }

    fn try_rfold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        self.iter_try_rfold(acc, move |acc, iter| {
            iter.map_err_mut(From::from).try_rfold(acc, &mut f)
        })
    }
}

impl<I, U> FusedTryIterator for Flatten<I, U>
where
    I: FusedTryIterator,
    U: TryIterator,
    I::Item: IntoTryIterator<Item = U::Item, Error = U::Error, IntoTryIter = U>,
    I::Error: From<U::Error>,
{
}
