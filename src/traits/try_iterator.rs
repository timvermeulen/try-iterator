use super::*;

use std::iter::{FromIterator, Product, Sum};

pub trait TryIterator {
    type Item;
    type Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    fn nth(&mut self, n: usize) -> Result<Option<Self::Item>, Self::Error> {
        self.try_nth(n).map(|x| x.ok())
    }

    fn try_nth(&mut self, mut n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        while let Some(e) = self.next()? {
            if n == 0 {
                return Ok(Ok(e));
            }
            n -= 1;
        }
        Ok(Err(n))
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
        R: Try<Ok = Option<T>>,
        R::Error: From<Self::Error>,
    {
        self.try_for_each(|x| match f(x).into_result() {
            Ok(None) => LoopState::Continue(()),
            Ok(Some(x)) => LoopState::Break(Some(x)),
            Err(e) => LoopState::MapError(e),
        })
        .map_continue(|()| None)
        .into_try()
    }

    fn find<F>(&mut self, f: F) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> bool,
    {
        self.try_find(FnWrapper::new(f))
    }

    fn try_find<F, R>(&mut self, mut f: F) -> Result<Option<Self::Item>, R::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = bool>,
        R::Error: From<Self::Error>,
    {
        self.try_find_map(|x| Ok(if f(&x)? { Some(x) } else { None }))
    }

    fn position<F>(&mut self, f: F) -> Result<Option<usize>, Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        self.try_position(FnWrapper::new(f))
    }

    fn try_position<F, R>(&mut self, mut f: F) -> Result<Option<usize>, R::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = bool>,
        R::Error: From<Self::Error>,
    {
        let mut n = 0;
        self.try_find_map(|x| {
            Ok(if f(x)? {
                Some(n)
            } else {
                n += 1;
                None
            })
        })
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
        R: Try<Ok = bool>,
        R::Error: From<Self::Error>,
    {
        let x: Result<_, R::Error> = self.try_find_map(|x| Ok(if f(x)? { Some(()) } else { None }));
        Try::from_ok(x?.is_some())
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
        R: Try<Ok = bool>,
        R::Error: From<Self::Error>,
    {
        self.try_any(|x| Try::from_ok(!f(x)?))
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

    fn partial_cmp_by<I, F>(self, other: I, f: F) -> Result<Option<Ordering>, Self::Error>
    where
        Self: Sized,
        I: IntoTryIterator,
        F: FnMut(Self::Item, I::Item) -> Option<Ordering>,
        Self::Error: From<I::Error>,
    {
        self.try_partial_cmp_by(other, FnWrapper::new(f))
    }

    fn try_partial_cmp_by<I, F, R>(mut self, other: I, mut f: F) -> R
    where
        Self: Sized,
        I: IntoTryIterator,
        F: FnMut(Self::Item, I::Item) -> R,
        R: Try<Ok = Option<Ordering>>,
        R::Error: From<Self::Error> + From<I::Error>,
    {
        let mut other = other.into_try_iter();
        self.try_for_each(|x| match other.next().into_result() {
            Ok(None) => LoopState::Break(Some(Ordering::Greater)),
            Ok(Some(y)) => match f(x, y).into_result() {
                Ok(Some(Ordering::Equal)) => LoopState::Continue(()),
                Ok(non_eq) => LoopState::Break(non_eq),
                Err(e) => LoopState::MapError(e),
            },
            Err(e) => LoopState::MapError(e.into()),
        })
        .try_map_continue(|()| match other.next().into_result() {
            Ok(None) => Ok(Some(Ordering::Equal)),
            Ok(Some(_)) => Ok(Some(Ordering::Less)),
            Err(e) => Err(LoopBreak::MapError(e.into())),
        })
        .into_try()
    }

    fn cmp_by<I, F>(self, other: I, f: F) -> Result<Ordering, Self::Error>
    where
        Self: Sized,
        I: IntoTryIterator,
        F: FnMut(Self::Item, I::Item) -> Ordering,
        Self::Error: From<I::Error>,
    {
        self.try_cmp_by(other, FnWrapper::new(f))
    }

    fn try_cmp_by<I, F, R>(mut self, other: I, mut f: F) -> R
    where
        Self: Sized,
        I: IntoTryIterator,
        F: FnMut(Self::Item, I::Item) -> R,
        R: Try<Ok = Ordering>,
        R::Error: From<Self::Error> + From<I::Error>,
    {
        let mut other = other.into_try_iter();
        self.try_for_each(|x| match other.next().into_result() {
            Ok(None) => LoopState::Break(Ordering::Greater),
            Ok(Some(y)) => match f(x, y).into_result() {
                Ok(Ordering::Equal) => LoopState::Continue(()),
                Ok(non_eq) => LoopState::Break(non_eq),
                Err(e) => LoopState::MapError(e),
            },
            Err(e) => LoopState::MapError(e.into()),
        })
        .try_map_continue(|()| match other.next().into_result() {
            Ok(None) => Ok(Ordering::Equal),
            Ok(Some(_)) => Ok(Ordering::Less),
            Err(e) => Err(LoopBreak::MapError(e.into())),
        })
        .into_try()
    }

    fn eq_by<I, F>(self, other: I, f: F) -> Result<bool, Self::Error>
    where
        Self: Sized,
        I: IntoTryIterator,
        F: FnMut(Self::Item, I::Item) -> bool,
        Self::Error: From<I::Error>,
    {
        self.try_eq_by(other, FnWrapper::new(f))
    }

    fn try_eq_by<I, F, R>(mut self, other: I, mut f: F) -> R
    where
        Self: Sized,
        I: IntoTryIterator,
        F: FnMut(Self::Item, I::Item) -> R,
        R: Try<Ok = bool>,
        R::Error: From<Self::Error> + From<I::Error>,
    {
        let mut other = other.into_try_iter();
        self.try_for_each(|x| match other.next().into_result() {
            Ok(None) => LoopState::Break(false),
            Ok(Some(y)) => match f(x, y).into_result() {
                Ok(true) => LoopState::Continue(()),
                Ok(false) => LoopState::Break(false),
                Err(e) => LoopState::MapError(e),
            },
            Err(e) => LoopState::MapError(e.into()),
        })
        .try_map_continue(|()| match other.next().into_result() {
            Ok(x) => Ok(x.is_none()),
            Err(e) => Err(LoopBreak::MapError(e.into())),
        })
        .into_try()
    }

    fn partial_cmp<I>(self, other: I) -> Result<Option<Ordering>, Self::Error>
    where
        Self: Sized,
        I: IntoTryIterator,
        Self::Item: PartialOrd<I::Item>,
        Self::Error: From<I::Error>,
    {
        self.partial_cmp_by(other, |a, b| a.partial_cmp(&b))
    }

    fn cmp<I>(self, other: I) -> Result<Ordering, Self::Error>
    where
        Self: Sized,
        I: IntoTryIterator<Item = Self::Item>,
        Self::Item: Ord,
        Self::Error: From<I::Error>,
    {
        self.cmp_by(other, |a, b| a.cmp(&b))
    }

    fn eq<I>(self, other: I) -> Result<bool, Self::Error>
    where
        Self: Sized,
        I: IntoTryIterator,
        Self::Item: PartialEq<I::Item>,
        Self::Error: From<I::Error>,
    {
        self.eq_by(other, |a, b| a.eq(&b))
    }

    fn lt<I>(self, other: I) -> Result<bool, Self::Error>
    where
        Self: Sized,
        I: IntoTryIterator,
        Self::Item: PartialOrd<I::Item>,
        Self::Error: From<I::Error>,
    {
        Ok(self.partial_cmp(other)? == Some(Ordering::Less))
    }

    fn le<I>(self, other: I) -> Result<bool, Self::Error>
    where
        Self: Sized,
        I: IntoTryIterator,
        Self::Item: PartialOrd<I::Item>,
        Self::Error: From<I::Error>,
    {
        Ok(match self.partial_cmp(other)? {
            Some(Ordering::Less) | Some(Ordering::Equal) => true,
            _ => false,
        })
    }

    fn gt<I>(self, other: I) -> Result<bool, Self::Error>
    where
        Self: Sized,
        I: IntoTryIterator,
        Self::Item: PartialOrd<I::Item>,
        Self::Error: From<I::Error>,
    {
        Ok(self.partial_cmp(other)? == Some(Ordering::Greater))
    }

    fn ge<I>(self, other: I) -> Result<bool, Self::Error>
    where
        Self: Sized,
        I: IntoTryIterator,
        Self::Item: PartialOrd<I::Item>,
        Self::Error: From<I::Error>,
    {
        Ok(match self.partial_cmp(other)? {
            Some(Ordering::Greater) | Some(Ordering::Equal) => true,
            _ => false,
        })
    }

    fn ne<I>(self, other: I) -> Result<bool, Self::Error>
    where
        Self: Sized,
        I: IntoTryIterator,
        Self::Item: PartialEq<I::Item>,
        Self::Error: From<I::Error>,
    {
        Ok(!self.eq(other)?)
    }

    fn is_sorted(self) -> Result<bool, Self::Error>
    where
        Self: Sized,
        Self::Item: PartialOrd,
    {
        self.is_sorted_by(|x, y| x <= y)
    }

    fn is_sorted_by<F>(self, f: F) -> Result<bool, Self::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        self.try_is_sorted_by(FnWrapper::new(f))
    }

    fn try_is_sorted_by<F, R>(mut self, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> R,
        R: Try<Ok = bool>,
        R::Error: From<Self::Error>,
    {
        let first = match self.next()? {
            Some(x) => x,
            None => return Try::from_ok(true),
        };

        self.map_err(R::Error::from)
            .try_fold(first, |x, y| match f(&x, &y)? {
                true => LoopState::Continue(y),
                false => LoopState::Break(false),
            })
            .map_continue(|_| true)
            .into_try()
    }

    fn is_sorted_by_key<F, K>(self, f: F) -> Result<bool, Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> K,
        K: PartialOrd,
    {
        self.try_is_sorted_by_key(FnWrapper::new(f))
    }

    fn try_is_sorted_by_key<F, R, K>(self, f: F) -> Result<bool, R::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = K>,
        R::Error: From<Self::Error>,
        K: PartialOrd,
    {
        self.try_map(f).is_sorted()
    }

    fn collect<B>(self) -> Result<B, Self::Error>
    where
        Self: Sized,
        B: FromIterator<Self::Item>,
    {
        self.into_results().collect()
    }

    fn sum<B>(self) -> Result<B, Self::Error>
    where
        Self: Sized,
        B: Sum<Self::Item>,
    {
        self.into_results().sum()
    }

    fn product<B>(self) -> Result<B, Self::Error>
    where
        Self: Sized,
        B: Product<Self::Item>,
    {
        self.into_results().product()
    }

    fn partition<B, F, R>(self, f: F) -> Result<(B, B), Self::Error>
    where
        Self: Sized,
        B: Default + Extend<Self::Item>,
        F: FnMut(&Self::Item) -> bool,
    {
        self.try_partition(FnWrapper::new(f))
    }

    fn try_partition<B, F, R>(mut self, mut f: F) -> Result<(B, B), R::Error>
    where
        Self: Sized,
        B: Default + Extend<Self::Item>,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = bool>,
        R::Error: From<Self::Error>,
    {
        self.try_fold((B::default(), B::default()), |(mut a, mut b), i| {
            if f(&i)? {
                a.extend(Some(i));
            } else {
                b.extend(Some(i));
            }
            Ok((a, b))
        })
    }

    fn is_partitioned<F>(self, f: F) -> Result<bool, Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        self.try_is_partitioned(FnWrapper::new(f))
    }

    fn try_is_partitioned<F, R>(mut self, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = bool>,
        R::Error: From<Self::Error>,
    {
        try { self.try_all(&mut f)? || !self.try_any(f)? }
    }

    fn unzip<A, B, FromA, FromB>(self) -> Result<(FromA, FromB), Self::Error>
    where
        Self: Sized + TryIterator<Item = (A, B)>,
        FromA: Default + Extend<A>,
        FromB: Default + Extend<B>,
    {
        self.fold(
            (Default::default(), Default::default()),
            |(mut xs, mut ys), (x, y)| {
                xs.extend(Some(x));
                ys.extend(Some(y));
                (xs, ys)
            },
        )
    }

    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }

    fn filter<F>(self, f: F) -> Filter<Self, FnWrapper<F, Self::Error>>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> bool,
    {
        self.try_filter(FnWrapper::new(f))
    }

    fn try_filter<F, R>(self, f: F) -> Filter<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = bool>,
        R::Error: From<Self::Error>,
    {
        Filter::new(self, f)
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
        R: Try<Ok = ()>,
        R::Error: From<Self::Error>,
    {
        Inspect::new(self, f)
    }

    fn map<F, T>(self, f: F) -> Map<Self, FnWrapper<F, Self::Error>>
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

    fn zip<I>(self, other: I) -> Zip<Self, I>
    where
        Self: Sized,
        I: TryIterator,
        Self::Error: From<I::Error>,
    {
        Zip::new(self, other)
    }

    fn chain<I>(self, other: I) -> Chain<Self, I>
    where
        Self: Sized,
        I: TryIterator<Item = Self::Item>,
        Self::Error: From<I::Error>,
    {
        Chain::new(self, other)
    }

    fn take(self, n: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, n)
    }

    fn take_while<F>(self, f: F) -> TakeWhile<Self, FnWrapper<F, Self::Error>>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> bool,
    {
        self.try_take_while(FnWrapper::new(f))
    }

    fn try_take_while<F, R>(self, f: F) -> TakeWhile<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = bool>,
        R::Error: From<Self::Error>,
    {
        TakeWhile::new(self, f)
    }

    fn take_while_map<F, T>(self, f: F) -> TakeWhileMap<Self, FnWrapper<F, Self::Error>>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<T>,
    {
        self.try_take_while_map(FnWrapper::new(f))
    }

    fn try_take_while_map<F, R, T>(self, f: F) -> TakeWhileMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = Option<T>>,
        R::Error: From<Self::Error>,
    {
        TakeWhileMap::new(self, f)
    }

    fn skip(self, n: usize) -> Skip<Self>
    where
        Self: Sized,
    {
        Skip::new(self, n)
    }

    fn skip_while<F>(self, f: F) -> SkipWhile<Self, FnWrapper<F, Self::Error>>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> bool,
    {
        self.try_skip_while(FnWrapper::new(f))
    }

    fn try_skip_while<F, R>(self, f: F) -> SkipWhile<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = bool>,
        R::Error: From<Self::Error>,
    {
        SkipWhile::new(self, f)
    }

    fn scan<St, F, R, T>(self, state: St, f: F) -> Scan<Self, St, FnWrapper<F, Self::Error>>
    where
        Self: Sized,
        F: FnMut(&mut St, Self::Item) -> Option<T>,
    {
        Scan::new(self, state, FnWrapper::new(f))
    }

    fn try_scan<St, F, R, T>(self, state: St, f: F) -> Scan<Self, St, F>
    where
        Self: Sized,
        F: FnMut(&mut St, Self::Item) -> R,
        R: Try<Ok = Option<T>>,
        R::Error: From<Self::Error>,
    {
        Scan::new(self, state, f)
    }

    fn cycle(self) -> Cycle<Self>
    where
        Self: Sized + Clone,
    {
        Cycle::new(self)
    }

    fn cloned<'a, T>(self) -> Cloned<Self>
    where
        Self: Sized + TryIterator<Item = &'a T>,
        T: Clone + 'a,
    {
        Cloned::new(self)
    }

    fn copied<'a, T>(self) -> Copied<Self>
    where
        Self: Sized + TryIterator<Item = &'a T>,
        T: Copy + 'a,
    {
        Copied::new(self)
    }

    fn enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate::new(self)
    }

    fn fuse(self) -> Fuse<Self>
    where
        Self: Sized,
    {
        Fuse::new(self)
    }

    fn peekable(self) -> Peekable<Self>
    where
        Self: Sized,
    {
        Peekable::new(self)
    }

    fn step_by(self, n: usize) -> StepBy<Self>
    where
        Self: Sized,
    {
        StepBy::new(self, n)
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

    fn into_results(self) -> IntoResults<Self>
    where
        Self: Sized,
    {
        IntoResults::new(self)
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

impl<I> TryIterator for &mut I
where
    I: TryIterator + ?Sized,
{
    type Item = I::Item;
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        (**self).next()
    }
}
