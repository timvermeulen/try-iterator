use super::*;

type SizeHint = (usize, Option<usize>);

pub const ZERO: SizeHint = (0, Some(0));

pub trait SizeHintExt {
    fn without_lower_bound(self) -> Self;
}

impl SizeHintExt for SizeHint {
    fn without_lower_bound(self) -> Self {
        let (_, upper) = self;
        (0, upper)
    }
}

pub trait SizeHintAdd {
    fn add(size_hint: SizeHint, x: Self) -> SizeHint;
}

impl SizeHintAdd for SizeHint {
    fn add((x_lower, x_upper): SizeHint, (y_lower, y_upper): SizeHint) -> SizeHint {
        (x_lower.saturating_add(y_lower), try { x_upper?.checked_add(y_upper?)? })
    }
}

impl SizeHintAdd for usize {
    fn add((lower, upper): SizeHint, n: usize) -> SizeHint {
        (lower.saturating_add(n), try { upper?.checked_add(n)? })
    }
}

pub fn add<T>(size_hint: SizeHint, x: T) -> SizeHint
where T: SizeHintAdd {
    T::add(size_hint, x)
}

pub trait SizeHintSub {
    fn sub(size_hint: SizeHint, x: Self) -> SizeHint;
}

impl SizeHintSub for SizeHint {
    fn sub((x_lower, x_upper): SizeHint, (y_lower, y_upper): SizeHint) -> SizeHint {
        (x_lower.saturating_sub(y_lower), try { x_upper?.saturating_sub(y_upper?) })
    }
}

impl SizeHintSub for usize {
    fn sub((lower, upper): SizeHint, n: usize) -> SizeHint {
        (lower.saturating_add(n), try { upper?.saturating_sub(n) })
    }
}

pub fn sub<T>(size_hint: SizeHint, x: T) -> SizeHint
where T: SizeHintSub {
    T::sub(size_hint, x)
}

pub trait SizeHintMin {
    fn min(size_hint: SizeHint, x: Self) -> SizeHint;
}

impl SizeHintMin for SizeHint {
    fn min((x_lower, x_upper): SizeHint, (y_lower, y_upper): SizeHint) -> SizeHint {
        (
            cmp::min(x_lower, y_lower),
            match (x_upper, y_upper) {
                (Some(x), Some(y)) => Some(cmp::min(x, y)),
                _ => x_upper.or(y_upper),
            },
        )
    }
}

impl SizeHintMin for usize {
    fn min((lower, upper): SizeHint, n: usize) -> SizeHint {
        (
            cmp::min(lower, n),
            Some(match upper {
                None => n,
                Some(x) => cmp::min(x, n),
            }),
        )
    }
}

pub fn min<T>(size_hint: SizeHint, x: T) -> SizeHint
where T: SizeHintMin {
    T::min(size_hint, x)
}
