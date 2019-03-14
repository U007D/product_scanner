#[cfg(test)]
mod unit_tests;

use std::num::NonZeroUsize;

pub trait NonZeroUsizeExt {
    fn checked_add(&self, rhs: usize) -> Option<Self> where Self: Sized;
}

impl NonZeroUsizeExt for NonZeroUsize {
    fn checked_add(&self, rhs: usize) -> Option<Self> {
        self.get()
            .checked_add(rhs)
            .and_then(|v| NonZeroUsize::new(v))
    }
}
