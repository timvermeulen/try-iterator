#![feature(
    try_trait,
    fn_traits,
    never_type,
    unboxed_closures,
    try_blocks,
    mem_take
)]

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
use loopstate::{LoopBreak, LoopState};
use size_hint::SizeHintExt;

use std::cmp::{self, Ordering};
use std::marker::PhantomData;
use std::mem;
use std::ops::Try;

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
        match self {
            None => Ok(None),
            Some(x) => Ok(Some(f(x)?)),
        }
    }
}
