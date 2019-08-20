use super::*;

pub struct TakeWhile<I, F> {
    iter: I,
    f: F,
    flag: bool,
}

impl<I, F, R> TakeWhile<I, F>
where
    I: TryIterator,
    F: FnMut(&I::Item) -> R,
    R: Try<Ok = bool>,
    R::Error: From<I::Error>,
{
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self {
            iter,
            f,
            flag: false,
        }
    }
}

impl<I, F, R> TryIterator for TakeWhile<I, F>
where
    I: TryIterator,
    F: FnMut(&I::Item) -> R,
    R: Try<Ok = bool>,
    R::Error: From<I::Error>,
{
    type Item = I::Item;
    type Error = R::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.flag {
            size_hint::ZERO
        } else {
            self.iter.size_hint().without_lower_bound()
        }
    }

    fn try_fold<Acc, G, Q>(&mut self, acc: Acc, mut g: G) -> Q
    where
        G: FnMut(Acc, Self::Item) -> Q,
        Q: Try<Ok = Acc>,
        Q::Error: From<Self::Error>,
    {
        if self.flag {
            return Try::from_ok(acc);
        }

        let f = &mut self.f;
        let flag = &mut self.flag;
        self.iter
            .map_err_mut(R::Error::from)
            .try_fold(acc, |acc, x| {
                if f(&x)? {
                    LoopState::continue_with_try(g(acc, x))
                } else {
                    *flag = true;
                    LoopState::Break(acc)
                }
            })
            .into_try()
    }
}
