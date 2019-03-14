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

use fraction::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Decimal, Bounded};

#[derive(Clone, Debug, Eq, PartialOrd, PartialEq)]
pub struct OrdDecimal(Decimal);

impl OrdDecimal {
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

impl CheckedAdd for OrdDecimal {
    fn checked_add(&self, other: &Self) -> Option<Self> {
        self.0.checked_add(&other.0).and_then(|v| Some(Self(v)))
    }
}

impl Sub for OrdDecimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0.sub(other.0))
    }
}

impl CheckedSub for OrdDecimal {
    fn checked_sub(&self, other: &Self) -> Option<Self> {
        self.0.checked_sub(&other.0).and_then(|v| Some(Self(v)))
    }
}

impl Mul for OrdDecimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0.mul(other.0))
    }
}

impl CheckedMul for OrdDecimal {
    fn checked_mul(&self, other: &Self) -> Option<Self> {
        self.0.checked_mul(&other.0).and_then(|v| Some(Self(v)))
    }
}

impl Div for OrdDecimal {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.0.div(other.0))
    }
}

impl CheckedDiv for OrdDecimal {
    fn checked_div(&self, other: &Self) -> Option<Self> {
        self.0.checked_div(&other.0).and_then(|v| Some(Self(v)))
    }
}
