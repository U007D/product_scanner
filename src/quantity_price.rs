use crate::{
    Decimal,
    Error,
    Result,
    Op,
};
use std::num::NonZeroUsize;
use fraction::CheckedDiv;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct QuantityPrice {
    pub(crate) unit_price: Decimal,
    pub(crate) quantity: NonZeroUsize,
    pub(crate) price: Decimal,
}

impl QuantityPrice {
    pub fn new(price: Decimal, quantity: NonZeroUsize) -> Result<Self> {
        Ok(Self {
            unit_price: price.checked_div(&Decimal::from(usize::from(quantity)))
                             .ok_or_else(|| Error::DecimalOverflow(Op::Div(price.clone(),
                                                                           Decimal::from(usize::from(quantity)))))?,
            quantity,
            price,
        })
    }
}

