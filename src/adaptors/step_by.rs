use super::*;

pub struct StepBy<I> {
    iter: I,
    n: usize,
    first_take: bool,
}

impl<I> StepBy<I>
where
    I: TryIterator,
{
    pub(crate) fn new(iter: I, n: usize) -> Self {
        assert!(n != 0);
        Self {
            iter,
            n: n - 1,
            first_take: true,
        }
    }
}

impl<I> TryIterator for StepBy<I>
where
    I: TryIterator,
{
    type Item = I::Item;
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        let f = |n| n / (self.n + 1);

        if self.first_take {
            let f = |n| {
                if n == 0 {
                    0
                } else {
                    1 + f(n - 1)
                }
            };
            (f(lower), upper.map(f))
        } else {
            (f(lower), upper.map(f))
        }
    }

    fn try_fold<Acc, F, R>(&mut self, mut acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        if self.first_take {
            self.first_take = false;
            match self.iter.next()? {
                None => return Try::from_ok(acc),
                Some(x) => acc = f(acc, x)?,
            }
        }

        from_fn(|| self.iter.nth(self.n)).try_fold(acc, f)
    }
}

impl<I> ExactSizeTryIterator for StepBy<I> where I: ExactSizeTryIterator {}
