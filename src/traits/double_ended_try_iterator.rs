use super::*;

pub trait DoubleEndedTryIterator: TryIterator {
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error>;

    fn nth_back(&mut self, n: usize) -> Result<Option<Self::Item>, Self::Error> {
        self.try_nth(n).map(|x| x.ok())
    }

    fn try_nth_back(&mut self, mut n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        while let Some(e) = self.next_back()? {
            if n == 0 {
                return Ok(Ok(e));
            }
            n -= 1;
        }
        Ok(Err(n))
    }

    fn rfold<Acc, F>(mut self, acc: Acc, mut f: F) -> Result<Acc, Self::Error>
    where
        Self: Sized,
        F: FnMut(Acc, Self::Item) -> Acc,
    {
        self.try_rfold(acc, move |acc, x| Ok(f(acc, x)))
    }

    fn try_rfold<Acc, F, R>(&mut self, mut acc: Acc, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        while let Some(v) = self.next_back()? {
            acc = f(acc, v)?;
        }
        Try::from_ok(acc)
    }

    fn rfor_each<F>(self, mut f: F) -> Result<(), Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item),
    {
        self.rfold((), |(), x| f(x))
    }

    fn try_rfor_each<F, R>(&mut self, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = ()>,
        R::Error: From<Self::Error>,
    {
        self.try_rfold((), |(), x| f(x))
    }

    fn rfind_map<F, T>(&mut self, f: F) -> Result<Option<T>, Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<T>,
    {
        self.try_rfind_map(FnWrapper::new(f))
    }

    fn try_rfind_map<F, R, T>(&mut self, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = Option<T>>,
        R::Error: From<Self::Error>,
    {
        self.try_rfor_each(|x| match f(x).into_result() {
            Ok(None) => LoopState::Continue(()),
            Ok(Some(x)) => LoopState::Break(Some(x)),
            Err(e) => LoopState::MapError(e),
        })
        .map_continue(|()| None)
        .into_try()
    }

    fn rfind<F>(&mut self, f: F) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> bool,
    {
        self.try_rfind(FnWrapper::new(f))
    }

    fn try_rfind<F, R>(&mut self, mut f: F) -> Result<Option<Self::Item>, R::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = bool>,
        R::Error: From<Self::Error>,
    {
        self.try_rfind_map(|x| Ok(if f(&x)? { Some(x) } else { None }))
    }

    fn rposition<F>(&mut self, f: F) -> Result<Option<usize>, Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        self.try_rposition(FnWrapper::new(f))
    }

    fn try_rposition<F, R>(&mut self, mut f: F) -> Result<Option<usize>, R::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = bool>,
        R::Error: From<Self::Error>,
    {
        let mut n = 0;
        self.try_rfind_map(|x| {
            Ok(if f(x)? {
                Some(n)
            } else {
                n += 1;
                None
            })
        })
    }

    fn partition_in_place<'a, T: 'a, F>(self, f: F) -> Result<usize, Self::Error>
    where
        Self: Sized + DoubleEndedTryIterator<Item = &'a mut T>,
        F: FnMut(&T) -> bool,
    {
        self.try_partition_in_place(FnWrapper::new(f))
    }

    fn try_partition_in_place<'a, T: 'a, F, R>(mut self, mut f: F) -> Result<usize, R::Error>
    where
        Self: Sized + DoubleEndedTryIterator<Item = &'a mut T>,
        F: FnMut(&T) -> R,
        R: Try<Ok = bool>,
        R::Error: From<Self::Error>,
    {
        let mut f = |x: &&mut _| f(&**x);
        let mut true_count = 0;

        while let Some(head) = self.try_find(|x| {
            let p = f(x)?;
            true_count += p as usize;
            Ok::<_, R::Error>(!p)
        })? {
            if let Some(tail) = self.try_rfind(&mut f)? {
                mem::swap(head, tail);
                true_count += 1;
            } else {
                break;
            }
        }

        Try::from_ok(true_count)
    }

    fn rev(self) -> Rev<Self>
    where
        Self: Sized,
    {
        Rev::new(self)
    }
}

impl<I> DoubleEndedTryIterator for &mut I
where
    I: DoubleEndedTryIterator + ?Sized,
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        (**self).next_back()
    }
}
