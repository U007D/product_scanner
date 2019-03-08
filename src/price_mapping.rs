use crate::{
    Decimal,
    Error,
    Result,
    Op,
};
use std::num::NonZeroUsize;
use fraction::CheckedDiv;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PriceMapping {
    pub(crate) unit_price: Decimal,
    pub(crate) quantity: NonZeroUsize,
    pub(crate) price: Decimal,
}

impl PriceMapping {
    pub fn new(price: Decimal, quantity: NonZeroUsize) -> Result<Self> {
        Ok(Self {
            unit_price: price.checked_div(&Decimal::from(usize::from(quantity)))
                             .ok_or_else(|| Error::OpYieldedUnrepresentableDecimalValue(Op::Div(price.clone(),
                                                                                                Decimal::from(usize::from(quantity)))))?,
            quantity,
            price,
        })
    }
}

