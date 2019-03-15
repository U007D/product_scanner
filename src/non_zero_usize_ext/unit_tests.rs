#![allow(clippy::option_unwrap_used)]
use super::*;
use std::usize;

#[test]
fn checked_add_nominal() {
    assert_eq!(Some(NonZeroUsize::new(42).unwrap()), NonZeroUsize::new(41).unwrap().checked_add(1));
}

#[test]
fn checked_add_overflow() {
    assert_eq!(None, NonZeroUsize::new(usize::MAX).unwrap().checked_add(1));
}

