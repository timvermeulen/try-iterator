use super::*;

pub trait IntoTryIterator {
    type Item;
    type Error;
    type IntoTryIter: TryIterator<Item = Self::Item, Error = Self::Error>;

    fn into_try_iter(self) -> Self::IntoTryIter;
}

impl<I> IntoTryIterator for I
where
    I: TryIterator,
{
    type Item = <Self as TryIterator>::Item;
    type Error = <Self as TryIterator>::Error;
    type IntoTryIter = Self;

    fn into_try_iter(self) -> Self::IntoTryIter {
        self
    }
}

mod result {
    use super::*;

    impl<T, E> IntoTryIterator for Result<T, E> {
        type Item = T;
        type Error = E;
        type IntoTryIter = Iter<T, E>;

        fn into_try_iter(self) -> Self::IntoTryIter {
            Iter(Some(self))
        }
    }

    pub struct Iter<T, E>(Option<Result<T, E>>);

    impl<T, E> TryIterator for Iter<T, E> {
        type Item = T;
        type Error = E;

        fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
            self.0.take().transpose()
        }
    }
}

mod option {
    use super::*;
    use core::option::NoneError;

    impl<T> IntoTryIterator for Option<T> {
        type Item = T;
        type Error = NoneError;
        type IntoTryIter = Iter<T>;

        fn into_try_iter(self) -> Self::IntoTryIter {
            Iter(self.into_result().into_try_iter())
        }
    }

    pub struct Iter<T>(result::Iter<T, NoneError>);

    impl<T> TryIterator for Iter<T> {
        type Item = T;
        type Error = NoneError;

        fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
            self.0.next()
        }
    }
}
