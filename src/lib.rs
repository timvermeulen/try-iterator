#![feature(try_trait, fn_traits, never_type, unboxed_closures)]

mod fn_wrapper;
mod iterator_ext;
mod iterator_wrapper;
mod try_iterator;

pub use crate::try_iterator::*;
pub use fn_wrapper::FnWrapper;
pub use iterator_ext::IteratorExt;
pub use iterator_wrapper::IteratorWrapper;

use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ops::Try;
