use crate::{
    Decimal,
    Error,
    Result,
    Op,
};
use std::num::NonZeroU32;
use fraction::CheckedDiv;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PriceMapping {
    pub(crate) unit_price: Decimal,
    pub(crate) quantity: NonZeroU32,
    pub(crate) price: Decimal,
}

impl PriceMapping {
    pub fn new(price: Decimal, quantity: NonZeroU32) -> Result<Self> {
        Ok(Self {
            unit_price: price.checked_div(&Decimal::from(u32::from(quantity)))
                             .ok_or_else(|| Error::OpYieldedUnrepresentableValue(Op::Div(price.clone(),
                                                                                 Decimal::from(u32::from(quantity)))))?,
            quantity,
            price,
        })
    }
}

