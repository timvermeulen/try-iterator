use super::*;

#[derive(Clone, Debug)]
pub struct Take<I> {
    iter: I,
    n: usize,
}

impl<I> Take<I> {
    pub(crate) fn new(iter: I, n: usize) -> Self {
        Self { iter, n }
    }
}

impl<I> TryIterator for Take<I>
where
    I: TryIterator,
{
    type Item = I::Item;
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        size_hint::min(self.iter.size_hint(), self.n)
    }

    fn try_nth(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        if self.n > n {
            self.n -= n + 1;
            self.iter.try_nth(n)
        } else {
            let k = mem::take(&mut self.n);
            let n = match k {
                0 => n,
                k => match self.iter.try_nth(k - 1)? {
                    Ok(_) => n - k,
                    Err(m) => n - k + m + 1,
                },
            };
            Ok(Err(n))
        }
    }

    fn try_fold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        if self.n == 0 {
            return Try::from_ok(acc);
        }

        let n = &mut self.n;
        self.iter
            .try_fold(acc, move |acc, x| {
                *n -= 1;
                let r = f(acc, x);
                if *n == 0 {
                    LoopState::break_with_try(r)
                } else {
                    LoopState::continue_with_try(r)
                }
            })
            .into_try()
    }
}

impl<I> DoubleEndedTryIterator for Take<I>
where
    I: DoubleEndedTryIterator + ExactSizeTryIterator,
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.rfind(|_| true)
    }

    fn try_nth_back(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        let len = self.iter.len();
        if self.n > n {
            let m = len.saturating_sub(self.n) + n;
            self.n -= n + 1;
            self.iter.try_nth_back(m)
        } else {
            let n = match len {
                0 => n,
                len => match self.iter.try_nth_back(len - 1)? {
                    Ok(_) => n - self.n,
                    Err(k) => n - self.n + k + 1,
                },
            };
            Ok(Err(n))
        }
    }

    fn try_rfold<Acc, F, R>(&mut self, acc: Acc, f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        if self.n == 0 {
            Try::from_ok(acc)
        } else {
            let len = self.iter.len();
            if len > self.n && self.iter.nth_back(len - self.n - 1)?.is_none() {
                Try::from_ok(acc)
            } else {
                self.iter.try_rfold(acc, f)
            }
        }
    }
}

impl<I> ExactSizeTryIterator for Take<I> where I: ExactSizeTryIterator {}

impl<I> FusedTryIterator for Take<I> where I: FusedTryIterator {}
