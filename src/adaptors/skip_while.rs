use super::*;

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct SkipWhile<I, F> {
    iter: I,
    f: F,
    flag: bool,
}

impl<I, F> SkipWhile<I, F> {
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f, flag: false }
    }
}

impl<I, F, R> TryIterator for SkipWhile<I, F>
where
    I: TryIterator,
    F: FnMut(&I::Item) -> R,
    R: Try<Ok = bool>,
    R::Error: From<I::Error>,
{
    type Item = I::Item;
    type Error = R::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        let flag = &mut self.flag;
        let f = &mut self.f;
        self.iter.try_find(|x| {
            Ok(if *flag || !f(x)? {
                *flag = true;
                true
            } else {
                false
            })
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint().without_lower_bound()
    }

    fn try_fold<Acc, G, Q>(&mut self, acc: Acc, mut g: G) -> Q
    where
        G: FnMut(Acc, Self::Item) -> Q,
        Q: Try<Ok = Acc>,
        Q::Error: From<Self::Error>,
    {
        let acc = if self.flag {
            acc
        } else {
            match self.next()? {
                None => return Try::from_ok(acc),
                Some(x) => g(acc, x)?,
            }
        };

        self.iter.map_err_mut(From::from).try_fold(acc, g)
    }
}

impl<I, F, R> FusedTryIterator for SkipWhile<I, F>
where
    I: FusedTryIterator,
    F: FnMut(&I::Item) -> R,
    R: Try<Ok = bool>,
    R::Error: From<I::Error>,
{
}
