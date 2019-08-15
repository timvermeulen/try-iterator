use super::*;

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
        Ok(match self.next.take() {
            None => None,
            Some(x) => {
                self.next = (self.f)(&x)?;
                Some(x)
            }
        })
    }
}

pub fn successors<T, F, R>(first: Option<T>, f: F) -> Successors<T, F>
where
    F: FnMut(&T) -> R,
    R: Try<Ok = Option<T>>,
{
    Successors { next: first, f }
}
