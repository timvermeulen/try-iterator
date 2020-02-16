use super::*;

pub trait IteratorExt: Iterator {
    fn try_filter<F, R>(self, f: F) -> Filter<IteratorWrapper<Self, R::Error>, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = bool>,
    {
        IteratorWrapper::new(self).try_filter(f)
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

    fn try_take_while<F, R>(self, f: F) -> TakeWhile<IteratorWrapper<Self, R::Error>, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = bool>,
    {
        IteratorWrapper::new(self).try_take_while(f)
    }

    fn try_map_while<F, R, T>(self, f: F) -> MapWhile<IteratorWrapper<Self, R::Error>, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = Option<T>>,
    {
        IteratorWrapper::new(self).try_map_while(f)
    }

    fn try_skip_while<F, R>(self, f: F) -> SkipWhile<IteratorWrapper<Self, R::Error>, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = bool>,
    {
        IteratorWrapper::new(self).try_skip_while(f)
    }

    fn try_scan<St, F, R, T>(self, state: St, f: F) -> Scan<IteratorWrapper<Self, R::Error>, St, F>
    where
        Self: Sized,
        F: FnMut(&mut St, Self::Item) -> R,
        R: Try<Ok = Option<T>>,
    {
        IteratorWrapper::new(self).try_scan(state, f)
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

    fn try_position<F, R>(&mut self, f: F) -> Result<Option<usize>, R::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = bool>,
    {
        IteratorWrapper::new(self).try_position(f)
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

    fn try_partial_cmp_by<I, F, R>(self, other: I, f: F) -> R
    where
        Self: Sized,
        I: IntoTryIterator,
        F: FnMut(Self::Item, I::Item) -> R,
        R: Try<Ok = Option<Ordering>>,
        R::Error: From<I::Error>,
    {
        IteratorWrapper::new(self).try_partial_cmp_by(other, f)
    }

    fn try_cmp_by<I, F, R>(self, other: I, f: F) -> R
    where
        Self: Sized,
        I: IntoTryIterator,
        F: FnMut(Self::Item, I::Item) -> R,
        R: Try<Ok = Ordering>,
        R::Error: From<I::Error>,
    {
        IteratorWrapper::new(self).try_cmp_by(other, f)
    }

    fn try_eq_by<I, F, R>(self, other: I, f: F) -> R
    where
        Self: Sized,
        I: IntoTryIterator,
        F: FnMut(Self::Item, I::Item) -> R,
        R: Try<Ok = bool>,
        R::Error: From<I::Error>,
    {
        IteratorWrapper::new(self).try_eq_by(other, f)
    }

    fn try_is_sorted_by<F, R>(self, f: F) -> R
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> R,
        R: Try<Ok = bool>,
    {
        IteratorWrapper::new(self).try_is_sorted_by(f)
    }

    fn try_is_sorted_by_key<F, R, K>(self, f: F) -> Result<bool, R::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = K>,
        K: PartialOrd,
    {
        IteratorWrapper::new(self).try_is_sorted_by_key(f)
    }

    fn try_partition<B, F, R>(self, f: F) -> Result<(B, B), R::Error>
    where
        Self: Sized,
        B: Default + Extend<Self::Item>,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = bool>,
    {
        IteratorWrapper::new(self).try_partition(f)
    }

    fn try_partition_in_place<'a, T: 'a, F, R>(self, f: F) -> Result<usize, R::Error>
    where
        Self: Sized + DoubleEndedIterator<Item = &'a mut T>,
        F: FnMut(&T) -> R,
        R: Try<Ok = bool>,
    {
        IteratorWrapper::new(self).try_partition_in_place(f)
    }

    fn try_is_partitioned<F, R>(self, f: F) -> R
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = bool>,
    {
        IteratorWrapper::new(self).try_is_partitioned(f)
    }
}

impl<I> IteratorExt for I where I: Iterator {}
