use crate::{
    Error,
    Product,
    Result,
};
use std::{
    collections::HashMap,
    num::NonZeroU32,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PriceMapping {
    quantity: NonZeroU32,
    price: f64,
    unit_price: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PriceListBuilder {
    price_list: PriceList,
}

impl PriceListBuilder {
    pub fn new() -> Self {
        Self {
            price_list: PriceList::default(),
        }
    }

    pub fn set_pricing(&mut self, product: Product, price: f64, quantity: NonZeroU32) -> Result<&mut Self> {
        let unit_price = price / f64::from(quantity);
        if !unit_price.is_finite() {
            Err(Error::UnitPriceNotRepresentable(price, quantity))?
        };
        self.price_list.add_entry(product, quantity, price, unit_price);

        Ok(self)

    }

    pub fn build(self) -> Result<PriceList> {
        match self.len() {
            0 => Err(Error::EmptyProductPricingTable),
            _ => Ok(self.price_list),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PriceList {
    table: HashMap<Product, Vec<PriceMapping>>,
}

impl PriceList {
    pub fn new() -> Self {
        Self {
            table: HashMap::<Product, Vec<PriceMapping>>::new()
        }
    }

    fn add_entry(&mut self, product: Product, quantity: NonZeroU32, price: f64, unit_price: f64) -> &mut Self {
        let price_mapping = PriceMapping {
            quantity,
            price,
            unit_price,
        };
        self.table.get_mut(&product).and_then(|v| v.push(price_mapping))
            .or_else(|| self.table.insert())
    }

    #[inline]
    pub fn len(&self) -> usize { self.table.len() }
}
