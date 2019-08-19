use super::*;

pub struct Take<I> {
    iter: I,
    n: usize,
}

impl<I> Take<I>
where
    I: TryIterator,
{
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

impl<I> ExactSizeTryIterator for Take<I> where I: ExactSizeTryIterator {}
