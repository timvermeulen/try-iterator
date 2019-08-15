use super::*;

pub struct RepeatWith<F> {
    f: F,
}

impl<F, R> TryIterator for RepeatWith<F>
where
    F: FnMut() -> R,
    R: Try,
{
    type Item = R::Ok;
    type Error = R::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        Ok(Some((self.f)()?))
    }
}

pub fn repeat_with<F, R>(f: F) -> RepeatWith<F>
where
    F: FnMut() -> R,
    R: Try,
{
    RepeatWith { f }
}
