#[cfg(test)]
mod unit_tests;

use std::{
    cmp::Ordering,
    convert::From,
    ops::{
        Add,
        Deref,
        DerefMut,
        Div,
        Mul,
        Sub
    }
};

use fraction::{Decimal, Bounded};

/// Floating point values exhibit
/// [problematic behavior](https://dl.acm.org/citation.cfm?id=103163) such as the fact that two floats cannot be
/// reliably compared for equality, rounding error (e.g. `assert_eq!(0.1_f64 + 0.2_f64, 0.3_f64)` will assert false).
/// The `fraction` crate's `Decimal` type solves these issues.  Like standards-compliant floating point values,
/// `Decimal` values are not hashable.  `OrdDecimal` is a NewType over `Decimal` with:
///     * total ordering implemented (ie. can be safely hashed)
///     * checked arithmetic implemented
/// `OrdDecimal` is `type` aliased to `Decimal` in `main.rs` to serve as a drop-in replacement of the latter.
#[derive(Clone, Debug, Eq, PartialOrd, PartialEq)]
pub struct OrdDecimal(Decimal);

impl OrdDecimal {
    /// Constructor.
    pub fn from<T>(value: T) -> Self where Decimal: From<T> {
        Self(Decimal::from(value))
    }
}

impl Deref for OrdDecimal {
    type Target = Decimal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OrdDecimal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Ord for OrdDecimal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .map_or_else(|| match self.0.is_nan() && other.0.is_nan() {
                             true => Ordering::Equal,
                             false => match self.0.is_nan() {
                                 true => Ordering::Greater,
                                 false => Ordering::Less,
                             }
                         },
                         |ord| ord)
    }
}

impl Add for OrdDecimal {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0.add(other.0))
    }
}

impl Bounded for OrdDecimal {
    fn min_value() -> Self {
        OrdDecimal(Decimal::min_value())
    }

    fn max_value() -> Self {
        OrdDecimal(Decimal::max_value())
    }
}

impl Sub for OrdDecimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0.sub(other.0))
    }
}

impl Mul for OrdDecimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0.mul(other.0))
    }
}

impl Div for OrdDecimal {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.0.div(other.0))
    }
}
