#![no_std]
#![feature(try_trait, fn_traits, never_type, unboxed_closures, try_blocks, specialization)]

mod adaptors;
mod fn_wrapper;
mod iterator_ext;
mod iterator_wrapper;
mod loopstate;
mod size_hint;
mod traits;

pub use adaptors::*;
pub use iterator_ext::IteratorExt;
pub use traits::*;

use fn_wrapper::FnWrapper;
use iterator_wrapper::IteratorWrapper;
use loopstate::{LoopState, MapResult};
use size_hint::SizeHintExt;

use core::{
    cmp::{self, Ordering},
    fmt::{self, Debug, Formatter},
    iter::FusedIterator,
    marker::PhantomData,
    mem,
    ops::Try,
};

trait OptionExt<T> {
    fn try_map<F, R>(self, f: F) -> Result<Option<R::Ok>, R::Error>
    where
        F: FnOnce(T) -> R,
        R: Try;
}

impl<T> OptionExt<T> for Option<T> {
    fn try_map<F, R>(self, f: F) -> Result<Option<R::Ok>, R::Error>
    where
        F: FnOnce(T) -> R,
        R: Try,
    {
        Ok(match self {
            None => None,
            Some(x) => Some(f(x)?),
        })
    }
}

fn try_min_by<T, F, R>(x: T, y: T, f: F) -> Result<T, R::Error>
where
    F: FnOnce(&T, &T) -> R,
    R: Try<Ok = Ordering>,
{
    Ok(match f(&x, &y)? {
        Ordering::Less | Ordering::Equal => x,
        Ordering::Greater => y,
    })
}

fn try_max_by<T, F, R>(x: T, y: T, f: F) -> Result<T, R::Error>
where
    F: FnOnce(&T, &T) -> R,
    R: Try<Ok = Ordering>,
{
    Ok(match f(&x, &y)? {
        Ordering::Less | Ordering::Equal => y,
        Ordering::Greater => x,
    })
}
