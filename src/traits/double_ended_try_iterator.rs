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

    fn rfor_each<F>(self, f: F) -> Result<(), Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item),
    {
        self.rev().for_each(f)
    }

    fn try_rfor_each<F, R>(&mut self, f: F) -> R
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = ()>,
        R::Error: From<Self::Error>,
    {
        self.rev_mut().try_for_each(f)
    }

    fn rfind_map<F, T>(&mut self, f: F) -> Result<Option<T>, Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<T>,
    {
        self.try_rfind_map(FnWrapper::new(f))
    }

    fn try_rfind_map<F, R, T>(&mut self, f: F) -> R
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = Option<T>>,
        R::Error: From<Self::Error>,
    {
        self.rev_mut().try_find_map(f)
    }

    fn rfind<F>(&mut self, f: F) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> bool,
    {
        self.try_rfind(FnWrapper::new(f))
    }

    fn try_rfind<F, R>(&mut self, f: F) -> Result<Option<Self::Item>, R::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = bool>,
        R::Error: From<Self::Error>,
    {
        self.rev_mut().try_rfind(f)
    }

    fn rposition<F>(&mut self, f: F) -> Result<Option<usize>, Self::Error>
    where
        Self: Sized + ExactSizeTryIterator,
        F: FnMut(Self::Item) -> bool,
    {
        self.try_rposition(FnWrapper::new(f))
    }

    fn try_rposition<F, R>(&mut self, f: F) -> Result<Option<usize>, R::Error>
    where
        Self: Sized + ExactSizeTryIterator,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = bool>,
        R::Error: From<Self::Error>,
    {
        self.rev_mut()
            .try_position(f)
            .map(|x| x.map(|_| self.len()))
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
        let mut true_count = 0;
        let mut f = |x: &&mut _| {
            f(&**x).into_result().map(|x| {
                true_count += x as usize;
                x
            })
        };

        while let Some(head) = self.try_find(|x| f(x).map(|x| !x))? {
            if let Some(tail) = self.try_rfind(&mut f)? {
                mem::swap(head, tail);
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

    fn rev_mut<'a>(&'a mut self) -> RevMut<'a, Self>
    where
        Self: Sized,
    {
        RevMut::new(self)
    }
}

impl<I> DoubleEndedTryIterator for &mut I
where
    I: DoubleEndedTryIterator + ?Sized,
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        (**self).next_back()
    }

    fn try_nth_back(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        (**self).try_nth_back(n)
    }
}
