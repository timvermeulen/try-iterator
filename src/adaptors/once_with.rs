use super::*;

pub fn once_with<F, R>(f: F) -> OnceWith<F>
where
    F: FnOnce() -> R,
    R: Try,
{
    OnceWith { f: Some(f) }
}

pub struct OnceWith<F> {
    f: Option<F>,
}

impl<F, R> TryIterator for OnceWith<F>
where
    F: FnOnce() -> R,
    R: Try,
{
    type Item = R::Ok;
    type Error = R::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.f.take().map(|f| f().into_result()).transpose()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<F, R> DoubleEndedTryIterator for OnceWith<F>
where
    F: FnOnce() -> R,
    R: Try,
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.next()
    }
}

impl<F, R> ExactSizeTryIterator for OnceWith<F>
where
    F: FnOnce() -> R,
    R: Try,
{
    fn len(&self) -> usize {
        match self.f {
            None => 0,
            Some(_) => 1,
        }
    }
}
