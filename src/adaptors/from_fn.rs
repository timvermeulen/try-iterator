use super::*;

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct FromFn<F> {
    f: F,
}

pub fn from_fn<F, R, T>(f: F) -> FromFn<F>
where
    F: FnMut() -> R,
    R: Try<Ok = Option<T>>,
{
    FromFn { f }
}

impl<F, R, T> TryIterator for FromFn<F>
where
    F: FnMut() -> R,
    R: Try<Ok = Option<T>>,
{
    type Item = T;
    type Error = R::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        (self.f)().into_result()
    }
}
