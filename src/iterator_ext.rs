use super::*;

pub trait IteratorExt: Iterator {
    fn try_filter<P, R>(self, predicate: P) -> Filter<IteratorWrapper<Self, R::Error>, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> R,
        R: Try<Ok = bool>,
    {
        IteratorWrapper::new(self).try_filter(predicate)
    }

    fn try_inspect<F, R>(self, f: F) -> Inspect<IteratorWrapper<Self, R::Error>, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = ()>,
    {
        IteratorWrapper::new(self).try_inspect(f)
    }

    fn try_map<F, R>(self, f: F) -> Map<IteratorWrapper<Self, R::Error>, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try,
    {
        IteratorWrapper::new(self).try_map(f)
    }

    fn try_filter_map<F, R, T>(self, f: F) -> FilterMap<IteratorWrapper<Self, R::Error>, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = Option<T>>,
    {
        IteratorWrapper::new(self).try_filter_map(f)
    }

    fn try_flat_map<F, R, U>(
        self,
        f: F,
    ) -> Flatten<Map<IteratorWrapper<Self, U::Error>, F>, U::IntoTryIter>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = U>,
        U: IntoTryIterator,
        R::Error: From<U::Error>,
    {
        IteratorWrapper::new(self).try_flat_map(f)
    }

    fn try_find_map<F, R, T>(&mut self, f: F) -> R
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = Option<T>>,
    {
        IteratorWrapper::new(self).try_find_map(f)
    }

    fn try_find<F, R>(&mut self, f: F) -> Result<Option<Self::Item>, R::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = bool>,
    {
        IteratorWrapper::new(self).try_find(f)
    }

    fn try_any<F, R>(&mut self, f: F) -> R
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = bool>,
    {
        IteratorWrapper::new(self).try_any(f)
    }

    fn try_all<F, R>(&mut self, f: F) -> R
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = bool>,
    {
        IteratorWrapper::new(self).try_all(f)
    }

    fn try_min_by<F, R>(self, f: F) -> Result<Option<Self::Item>, R::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> R,
        R: Try<Ok = Ordering>,
    {
        IteratorWrapper::new(self).try_min_by(f)
    }

    fn try_min_by_key<F, R, T>(self, f: F) -> Result<Option<Self::Item>, R::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = T>,
        T: Ord,
    {
        IteratorWrapper::new(self).try_min_by_key(f)
    }

    fn try_max_by<F, R>(self, f: F) -> Result<Option<Self::Item>, R::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> R,
        R: Try<Ok = Ordering>,
    {
        IteratorWrapper::new(self).try_max_by(f)
    }

    fn try_max_by_key<F, R, T>(self, f: F) -> Result<Option<Self::Item>, R::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = T>,
        T: Ord,
    {
        IteratorWrapper::new(self).try_max_by_key(f)
    }
}

impl<I> IteratorExt for I where I: Iterator {}
