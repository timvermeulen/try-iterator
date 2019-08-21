use super::*;

pub trait FusedTryIterator: TryIterator {}

impl<I> FusedTryIterator for &mut I where I: FusedTryIterator + ?Sized {}
