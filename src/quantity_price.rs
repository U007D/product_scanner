use crate::{
    Decimal,
    Result,
};
use std::num::NonZeroUsize;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct QuantityPrice {
    pub(crate) unit_price: Decimal,
    pub(crate) quantity: NonZeroUsize,
    pub(crate) price: Decimal,
}

impl QuantityPrice {
    pub fn new(price: Decimal, quantity: NonZeroUsize) -> Result<Self> {
        Ok(Self {
            unit_price: price.clone() / Decimal::from(usize::from(quantity)),
            quantity,
            price,
        })
    }
}

