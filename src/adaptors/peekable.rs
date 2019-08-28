use super::*;

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
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

    fn count(self) -> Result<usize, Self::Error> {
        match self.peeked {
            None => self.iter.count(),
            Some(None) => Ok(0),
            Some(Some(_)) => Ok(self.iter.count()? + 1),
        }
    }

    fn last(mut self) -> Result<Option<Self::Item>, Self::Error> {
        match self.peeked.take() {
            None => self.iter.last(),
            Some(None) => Ok(None),
            Some(Some(x)) => Ok(Some(self.iter.last()?.unwrap_or(x))),
        }
    }
}

impl<I> DoubleEndedTryIterator for Peekable<I>
where
    I: DoubleEndedTryIterator,
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.rfind(|_| true)
    }

    fn try_nth_back(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        Ok(match self.peeked.take() {
            Some(None) => Err(n),
            Some(Some(x)) => match self.iter.try_nth(n)? {
                Ok(x) => Ok(x),
                Err(0) => Ok(x),
                Err(n) => Err(n),
            },
            None => self.iter.try_nth(n)?,
        })
    }

    fn try_rfold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        match self.peeked.take() {
            Some(None) => return Try::from_ok(acc),
            Some(Some(x)) => match self.iter.try_rfold(acc, &mut f).into_result() {
                Ok(acc) => f(acc, x),
                Err(e) => {
                    self.peeked = Some(Some(x));
                    Try::from_error(e)
                }
            },
            None => self.iter.try_rfold(acc, f),
        }
    }
}

impl<I> ExactSizeTryIterator for Peekable<I> where I: ExactSizeTryIterator {}

impl<I> FusedTryIterator for Peekable<I> where I: FusedTryIterator {}
