//! Functional traits used by combinators.

/// Map a single value
pub trait MapOnce<T, U> {
    fn call(self, val: T) -> U;
}

impl<F, T, U> MapOnce<T, U> for F
where F: FnOnce(T) -> U
{
    fn call(self, val: T) -> U {
        self(val)
    }
}
