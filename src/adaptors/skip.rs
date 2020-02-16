use super::*;

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Skip<I> {
    iter: I,
    n: usize,
}

impl<I> Skip<I> {
    pub(crate) fn new(iter: I, n: usize) -> Self {
        Self { iter, n }
    }
}

impl<I> TryIterator for Skip<I>
where I: TryIterator
{
    type Item = I::Item;
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        size_hint::sub(self.iter.size_hint(), self.n)
    }

    fn try_nth(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        if self.n == 0 {
            self.iter.try_nth(n)
        } else {
            match self.iter.try_nth(mem::take(&mut self.n) - 1)? {
                Ok(_) => self.iter.try_nth(n),
                Err(_) => Ok(Err(n)),
            }
        }
    }

    fn try_fold<Acc, F, R>(&mut self, acc: Acc, f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        let n = self.n;
        self.n = 0;
        if n > 0 && self.iter.nth(n - 1)?.is_none() {
            Try::from_ok(acc)
        } else {
            self.iter.try_fold(acc, f)
        }
    }
}

impl<I> DoubleEndedTryIterator for Skip<I>
where I: DoubleEndedTryIterator + ExactSizeTryIterator
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.rfind(|_| true)
    }

    fn try_nth_back(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        let len = self.len();
        if n < len {
            self.iter.try_nth_back(n)
        } else {
            let n = match len {
                0 => n,
                len => match self.iter.try_nth_back(len - 1)? {
                    Ok(_) => n - len,
                    Err(k) => n - len + k + 1,
                },
            };
            Ok(Err(n))
        }
    }

    fn try_rfold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        let mut n = self.len();
        if n == 0 {
            Try::from_ok(acc)
        } else {
            self.iter
                .try_rfold(acc, move |acc, x| {
                    n -= 1;
                    let r = f(acc, x);
                    if n == 0 {
                        LoopState::break_with_try(r)
                    } else {
                        LoopState::continue_with_try(r)
                    }
                })
                .into_try()
        }
    }
}

impl<I> ExactSizeTryIterator for Skip<I> where I: ExactSizeTryIterator {}

impl<I> FusedTryIterator for Skip<I> where I: FusedTryIterator {}
