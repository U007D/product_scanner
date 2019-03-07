use crate::{
    Error,
    Result,
};
use std::num::NonZeroU32;

#[derive(Clone, Debug, PartialEq)]
pub struct PriceMapping {
    pub(crate) unit_price: f64,
    pub(crate) quantity: NonZeroU32,
    pub(crate) price: f64,
}

impl PriceMapping {
    pub fn new(price: f64, quantity: NonZeroU32) -> Result<Self> {
        Ok(Self {
            quantity,
            price,
            unit_price: {
                let unit_price = price / f64::from(u32::from(quantity));
                match unit_price.is_finite() {
                    true => unit_price,
                    false => Err(Error::UnitPriceNotRepresentable(price, quantity))?,
                }
            }
        })
    }
}
