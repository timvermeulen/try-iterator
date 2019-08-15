use super::*;

pub struct OnceWith<F> {
    f: Option<F>,
}

pub fn once_with<F, R>(f: F) -> OnceWith<F>
where
    F: FnOnce() -> R,
    R: Try,
{
    OnceWith { f: Some(f) }
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
}
