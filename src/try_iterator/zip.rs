use super::*;

pub struct Zip<A, B> {
    a: A,
    b: B,
}

impl<A, B> Zip<A, B>
where
    A: TryIterator,
    B: TryIterator,
    A::Error: From<B::Error>,
{
    pub(crate) fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A, B> TryIterator for Zip<A, B>
where
    A: TryIterator,
    B: TryIterator,
    A::Error: From<B::Error>,
{
    type Item = (A::Item, B::Item);
    type Error = A::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        match (self.a.next()?, self.b.next()?) {
            (Some(x), Some(y)) => Ok(Some((x, y))),
            _ => Ok(None),
        }
    }
}
