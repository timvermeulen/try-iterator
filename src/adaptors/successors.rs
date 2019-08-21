use super::*;

pub fn successors<T, F, R>(first: Option<T>, f: F) -> Successors<T, F>
where
    F: FnMut(&T) -> R,
    R: Try<Ok = Option<T>>,
{
    Successors { next: first, f }
}

#[derive(Clone, Debug)]
pub struct Successors<T, F> {
    next: Option<T>,
    f: F,
}

impl<T, F, R> TryIterator for Successors<T, F>
where
    F: FnMut(&T) -> R,
    R: Try<Ok = Option<T>>,
{
    type Item = T;
    type Error = R::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.next.take().try_map(|x| {
            self.next = (self.f)(&x)?;
            Ok(x)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.next {
            None => size_hint::ZERO,
            Some(_) => (1, None),
        }
    }
}

impl<T, F, R> FusedTryIterator for Successors<T, F>
where
    F: FnMut(&T) -> R,
    R: Try<Ok = Option<T>>,
{
}
