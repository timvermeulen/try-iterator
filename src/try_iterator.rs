use super::*;

mod enumerate;
mod filter;
mod filter_map;
mod flatten;
mod inspect;
mod map;
mod map_err;
mod map_err_mut;

pub use enumerate::Enumerate;
pub use filter::Filter;
pub use filter_map::FilterMap;
pub use flatten::Flatten;
pub use inspect::Inspect;
pub use map::Map;
pub use map_err::MapErr;
pub use map_err_mut::MapErrMut;

pub trait TryIterator {
    type Item;
    type Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error>;

    fn filter<P>(self, predicate: P) -> Filter<Self, FnWrapper<P, Self::Error>>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        self.try_filter(FnWrapper::new(predicate))
    }

    fn try_filter<P, R>(self, predicate: P) -> Filter<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> R,
        R: Try<Ok = bool>,
        R::Error: From<Self::Error>,
    {
        Filter::new(self, predicate)
    }

    fn inspect<F>(self, f: F) -> Inspect<Self, FnWrapper<F, Self::Error>>
    where
        Self: Sized,
        F: FnMut(&Self::Item),
    {
        self.try_inspect(FnWrapper::new(f))
    }

    fn try_inspect<F, R>(self, f: F) -> Inspect<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = (), Error = Self::Error>,
    {
        Inspect::new(self, f)
    }

    fn map<F, R, T>(self, f: F) -> Map<Self, FnWrapper<F, Self::Error>>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> T,
    {
        self.try_map(FnWrapper::new(f))
    }

    fn try_map<F, R>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try,
        R::Error: From<Self::Error>,
    {
        Map::new(self, f)
    }

    fn filter_map<F, R, T>(self, f: F) -> FilterMap<Self, FnWrapper<F, Self::Error>>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<T>,
    {
        self.try_filter_map(FnWrapper::new(f))
    }

    fn try_filter_map<F, R, T>(self, f: F) -> FilterMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = Option<T>>,
        R::Error: From<Self::Error>,
    {
        FilterMap::new(self, f)
    }

    fn flat_map<F, R, U>(
        self,
        f: F,
    ) -> Flatten<Map<Self, FnWrapper<F, Self::Error>>, U::IntoTryIter>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> U,
        U: IntoTryIterator,
        Self::Error: From<U::Error>,
    {
        self.try_flat_map(FnWrapper::new(f))
    }

    fn try_flat_map<F, R, U>(self, f: F) -> Flatten<Map<Self, F>, U::IntoTryIter>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = U>,
        U: IntoTryIterator,
        R::Error: From<Self::Error> + From<U::Error>,
    {
        self.try_map(f).flatten()
    }

    fn flatten<I>(self) -> Flatten<Self, I>
    where
        Self: Sized,
        Self::Item: IntoTryIterator<Item = I::Item, Error = I::Error, IntoTryIter = I>,
        Self::Error: From<I::Error>,
        I: TryIterator,
    {
        Flatten::new(self)
    }

    fn fold<Acc, F>(mut self, acc: Acc, mut f: F) -> Result<Acc, Self::Error>
    where
        Self: Sized,
        F: FnMut(Acc, Self::Item) -> Acc,
    {
        self.try_fold(acc, move |acc, x| Ok(f(acc, x)))
    }

    fn try_fold<Acc, F, R>(&mut self, mut acc: Acc, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        while let Some(v) = self.next()? {
            acc = f(acc, v)?;
        }
        Try::from_ok(acc)
    }

    fn for_each<F>(self, mut f: F) -> Result<(), Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item),
    {
        self.fold((), |(), x| f(x))
    }

    fn try_for_each<F, R>(&mut self, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = ()>,
        R::Error: From<Self::Error>,
    {
        self.try_fold((), |(), x| f(x))
    }

    fn find_map<F, T>(&mut self, f: F) -> Result<Option<T>, Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<T>,
    {
        self.try_find_map(FnWrapper::new(f))
    }

    fn try_find_map<F, R, T>(&mut self, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = Option<T>, Error = Self::Error>,
    {
        match self.try_for_each(|x| match f(x)? {
            None => Ok(()),
            Some(x) => Err(FoldStop::Break(x)),
        }) {
            Ok(()) => Try::from_ok(None),
            Err(FoldStop::Break(x)) => Try::from_ok(Some(x)),
            Err(FoldStop::Err(e)) => Try::from_error(e),
        }
    }

    fn find<F>(&mut self, f: F) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> bool,
    {
        self.try_find(FnWrapper::new(f))
    }

    fn try_find<F, R>(&mut self, mut f: F) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = bool, Error = Self::Error>,
    {
        self.try_find_map(|x| Ok(if f(&x)? { Some(x) } else { None }))
    }

    fn any<F, R>(&mut self, f: F) -> Result<bool, Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        self.try_any(FnWrapper::new(f))
    }

    fn try_any<F, R>(&mut self, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = bool, Error = Self::Error>,
    {
        Try::from_ok(
            self.try_find_map(|x| Ok(if f(x)? { Some(()) } else { None }))?
                .is_some(),
        )
    }

    fn all<F, R>(&mut self, f: F) -> Result<bool, Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        self.try_all(FnWrapper::new(f))
    }

    fn try_all<F, R>(&mut self, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = bool, Error = Self::Error>,
    {
        self.try_any(|x| Try::from_ok(!f(x)?))
    }

    fn enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate::new(self)
    }

    fn count(self) -> Result<usize, Self::Error>
    where
        Self: Sized,
    {
        self.fold(0, |n, _| n + 1)
    }

    fn last(self) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
    {
        self.fold(None, |_, x| Some(x))
    }

    fn nth(&mut self, mut n: usize) -> Result<Option<Self::Item>, Self::Error> {
        while let Some(e) = self.next()? {
            if n == 0 {
                return Ok(Some(e));
            }
            n -= 1;
        }
        Ok(None)
    }

    fn map_err<F, E>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Error) -> E,
    {
        MapErr::new(self, f)
    }

    fn map_err_mut<'a, F, E>(&'a mut self, f: F) -> MapErrMut<'a, Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Error) -> E,
    {
        MapErrMut::new(self, f)
    }

    fn min(self) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.min_by(Ord::cmp)
    }

    fn min_by<F>(self, f: F) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        self.try_min_by(FnWrapper::new(f))
    }

    fn try_min_by<F, R>(self, mut f: F) -> Result<Option<Self::Item>, R::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> R,
        R: Try<Ok = Ordering>,
        R::Error: From<Self::Error>,
    {
        try_select_fold1(self, |x, y| Ok(f(x, y)? == Ordering::Greater))
    }

    fn min_by_key<F, T>(self, f: F) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> T,
        T: Ord,
    {
        self.try_min_by_key(FnWrapper::new(f))
    }

    fn try_min_by_key<F, R, T>(self, mut f: F) -> Result<Option<Self::Item>, R::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = T>,
        R::Error: From<Self::Error>,
        T: Ord,
    {
        select_fold1(self.try_map(|x| Ok((f(&x)?, x))), |(x_p, _), (y_p, _)| {
            x_p > y_p
        })
        .map(|x| x.map(|(_, x)| x))
    }

    fn max(self) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.max_by(Ord::cmp)
    }

    fn max_by<F>(self, f: F) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        self.try_max_by(FnWrapper::new(f))
    }

    fn try_max_by<F, R>(self, mut f: F) -> Result<Option<Self::Item>, R::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> R,
        R: Try<Ok = Ordering>,
        R::Error: From<Self::Error>,
    {
        try_select_fold1(self, |x, y| Ok(f(x, y)? != Ordering::Greater))
    }

    fn max_by_key<F, T>(self, f: F) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> T,
        T: Ord,
    {
        self.try_max_by_key(FnWrapper::new(f))
    }

    fn try_max_by_key<F, R, T>(self, mut f: F) -> Result<Option<Self::Item>, R::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = T>,
        R::Error: From<Self::Error>,
        T: Ord,
    {
        select_fold1(self.try_map(|x| Ok((f(&x)?, x))), |(x_p, _), (y_p, _)| {
            x_p <= y_p
        })
        .map(|x| x.map(|(_, x)| x))
    }
}

fn select_fold1<I, F>(iter: I, f: F) -> Result<Option<I::Item>, I::Error>
where
    I: TryIterator,
    F: FnMut(&I::Item, &I::Item) -> bool,
{
    try_select_fold1(iter, FnWrapper::new(f))
}

fn try_select_fold1<I, F, R>(mut iter: I, mut f: F) -> Result<Option<I::Item>, R::Error>
where
    I: TryIterator,
    F: FnMut(&I::Item, &I::Item) -> R,
    R: Try<Ok = bool>,
    R::Error: From<I::Error>,
{
    let first = match iter.next()? {
        None => return Ok(None),
        Some(first) => first,
    };
    iter.try_fold(first, |sel, x| Ok(if f(&sel, &x)? { x } else { sel }))
        .map(Some)
}

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

enum FoldStop<T, E> {
    Break(T),
    Err(E),
}

impl<T, E> From<E> for FoldStop<T, E> {
    fn from(e: E) -> FoldStop<T, E> {
        FoldStop::Err(e)
    }
}
