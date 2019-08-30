use super::*;

pub enum LoopState<C, B, I, M> {
    Continue(C),
    Break(B),
    IterError(I),
    MapError(M),
}

impl<C, B, I, M> LoopState<C, B, I, M> {
    pub fn continue_with_try<R>(r: R) -> Self
    where
        R: Try<Ok = C, Error = M>,
    {
        match r.into_result() {
            Ok(x) => Self::Continue(x),
            Err(e) => Self::MapError(e),
        }
    }

    pub fn break_with_try<R>(r: R) -> Self
    where
        R: Try<Ok = B, Error = M>,
    {
        match r.into_result() {
            Ok(x) => Self::Break(x),
            Err(e) => Self::MapError(e),
        }
    }

    pub fn map_continue<F, T>(self, f: F) -> LoopState<T, B, I, M>
    where
        F: FnOnce(C) -> T,
    {
        self.try_map_continue(|x| Ok::<_, LoopBreak<B, I, M>>(f(x)))
    }

    pub fn try_map_continue<F, R, T>(self, f: F) -> LoopState<T, B, I, M>
    where
        F: FnOnce(C) -> R,
        R: Try<Ok = T, Error = LoopBreak<B, I, M>>,
    {
        LoopState::Continue(f(self?)?)
    }

    pub fn map_break<F, T>(self, f: F) -> LoopState<C, T, I, M>
    where
        F: FnOnce(B) -> T,
    {
        match self {
            Self::Continue(x) => LoopState::Continue(x),
            Self::Break(x) => LoopState::Break(f(x)),
            Self::IterError(e) => LoopState::IterError(e),
            Self::MapError(e) => LoopState::MapError(e),
        }
    }

    pub fn map_iter_error<F, E>(self, f: F) -> LoopState<C, B, E, M>
    where
        F: FnOnce(I) -> E,
    {
        match self {
            Self::Continue(x) => LoopState::Continue(x),
            Self::Break(x) => LoopState::Break(x),
            Self::IterError(e) => LoopState::IterError(f(e)),
            Self::MapError(e) => LoopState::MapError(e),
        }
    }
}

impl<T, I, M> LoopState<T, T, I, M>
where
    M: From<I>,
{
    pub fn into_try<R: Try<Ok = T, Error = M>>(self) -> R {
        match self {
            Self::Continue(x) | Self::Break(x) => Try::from_ok(x),
            Self::IterError(e) => Try::from_error(e.into()),
            Self::MapError(e) => Try::from_error(e),
        }
    }
}

impl<C, B, I, M> Try for LoopState<C, B, I, M> {
    type Ok = C;
    type Error = LoopBreak<B, I, M>;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Self::Continue(x) => Ok(x),
            Self::Break(x) => Err(LoopBreak::Value(x)),
            Self::IterError(e) => Err(LoopBreak::IterError(e)),
            Self::MapError(e) => Err(LoopBreak::MapError(e)),
        }
    }

    fn from_error(e: Self::Error) -> Self {
        match e {
            LoopBreak::Value(x) => Self::Break(x),
            LoopBreak::IterError(e) => Self::IterError(e),
            LoopBreak::MapError(e) => Self::MapError(e),
        }
    }

    fn from_ok(x: Self::Ok) -> Self {
        Self::Continue(x)
    }
}

pub enum LoopBreak<T, I, M> {
    Value(T),
    IterError(I),
    MapError(M),
}

impl<T, I, M> From<I> for LoopBreak<T, I, M> {
    fn from(e: I) -> Self {
        Self::IterError(e)
    }
}

impl<T, I, M> From<MapError<I, M>> for LoopBreak<T, I, M> {
    fn from(MapError { e, .. }: MapError<I, M>) -> Self {
        Self::MapError(e)
    }
}

pub struct MapResult<T, I, M> {
    inner: Result<T, M>,
    _marker: PhantomData<I>,
}

impl<T, I, M> MapResult<T, I, M> {
    fn new(inner: Result<T, M>) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    pub fn wrap<R>(r: R) -> Self
    where
        R: Try<Ok = T>,
        M: From<R::Error>,
    {
        Self::new(r.into_result().map_err(From::from))
    }
}

impl<T, I, M> Try for MapResult<T, I, M> {
    type Ok = T;
    type Error = MapError<I, M>;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        self.inner.map_err(|e| MapError {
            e,
            _marker: PhantomData,
        })
    }

    fn from_error(MapError { e, .. }: Self::Error) -> Self {
        Self::new(Err(e))
    }

    fn from_ok(x: Self::Ok) -> Self {
        Self::new(Ok(x))
    }
}

pub struct MapError<I, M> {
    e: M,
    _marker: PhantomData<I>,
}
