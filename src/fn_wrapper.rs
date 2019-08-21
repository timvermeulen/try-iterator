use super::*;

pub struct FnWrapper<F, E> {
    f: F,
    _marker: PhantomData<E>,
}

impl<F, E> FnWrapper<F, E> {
    pub(crate) fn new(f: F) -> Self {
        Self {
            f,
            _marker: PhantomData,
        }
    }
}

impl<F, E> Clone for FnWrapper<F, E>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.f.clone())
    }
}

impl<F, E> Debug for FnWrapper<F, E>
where
    F: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("FnWrapper").field("f", &self.f).finish()
    }
}

impl<Args, F, E> FnOnce<Args> for FnWrapper<F, E>
where
    F: FnOnce<Args>,
{
    type Output = Result<F::Output, E>;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        Ok(self.f.call_once(args))
    }
}

impl<Args, F, E> FnMut<Args> for FnWrapper<F, E>
where
    F: FnMut<Args>,
{
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        Ok(self.f.call_mut(args))
    }
}

impl<Args, F, E> Fn<Args> for FnWrapper<F, E>
where
    F: Fn<Args>,
{
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        Ok(self.f.call(args))
    }
}
