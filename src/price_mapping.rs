use crate::Result;
use ordered_float::NotNan;
use std::num::NonZeroU32;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PriceMapping {
    pub(crate) unit_price: NotNan<f64>,
    pub(crate) quantity: NonZeroU32,
    pub(crate) price: NotNan<f64>,
}

impl PriceMapping {
    pub fn new(price: NotNan<f64>, quantity: NonZeroU32) -> Result<Self> {
        Ok(Self {
            quantity,
            price,
            unit_price: NotNan::new(price.into_inner() / f64::from(u32::from(quantity)))?,
        })
    }
}
