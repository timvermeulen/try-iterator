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
                self.peeked = Some(self.iter.next()?);
                match self.peeked {
                    None => unreachable!(),
                    Some(ref x) => Ok(x.as_ref()),
                }
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let peeked_len = match self.peeked {
            None => 0,
            Some(None) => return size_hint::ZERO,
            Some(Some(_)) => 1,
        };
        size_hint::add(self.iter.size_hint(), peeked_len)
    }

    fn try_nth(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        match self.peeked.take() {
            Some(None) => Ok(Err(n)),
            Some(Some(x)) => {
                if n == 0 {
                    Ok(Ok(x))
                } else {
                    self.iter.try_nth(n - 1)
                }
            }
            None => self.iter.try_nth(n),
        }
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

impl<I> ExactSizeTryIterator for Peekable<I> where I: ExactSizeTryIterator {}
