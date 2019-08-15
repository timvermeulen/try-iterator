use super::*;

pub struct Peekable<I>
where
    I: TryIterator,
{
    iter: I,
    peeked: Option<Option<I::Item>>,
}

impl<I> Peekable<I>
where
    I: TryIterator,
{
    pub(crate) fn new(iter: I) -> Self {
        Self { iter, peeked: None }
    }

    pub fn peek(&mut self) -> Result<Option<&I::Item>, I::Error> {
        match self.peeked {
            Some(ref x) => Ok(x.as_ref()),
            None => {
                let x = self.iter.next()?;
                Ok(self.peeked.get_or_insert(x).as_ref())
            }
        }
    }
}

impl<I> TryIterator for Peekable<I>
where
    I: TryIterator,
{
    type Item = I::Item;
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn try_fold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        let acc = match self.peeked.take() {
            None => acc,
            Some(None) => return Try::from_ok(acc),
            Some(Some(x)) => f(acc, x)?,
        };
        self.iter.try_fold(acc, f)
    }
}
