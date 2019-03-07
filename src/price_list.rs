#[cfg(test)]
mod unit_tests;

use crate::{
    Error,
    price_mapping::PriceMapping,
    Product,
    Result,
};
use std::collections::HashMap;

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

    pub fn set_pricing(&mut self, product: Product, price_mapping: PriceMapping) -> &mut Self {
        self.price_list.add_entry(product, price_mapping);
        self
    }

    pub fn build(self) -> Result<PriceList> {
        match self.price_list.len() {
            0 => Err(Error::EmptyProductPricingTable),
            _ => Ok(self.price_list),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PriceList {
    map: HashMap<Product, Vec<PriceMapping>>,
}

impl PriceList {
    pub fn new() -> Self {
        Self {
            map: HashMap::<Product, Vec<PriceMapping>>::new()
        }
    }

    fn add_entry(&mut self, product: Product, price_mapping: PriceMapping) -> &mut Self {
        self.map.entry(product)
            .or_insert_with(Vec::<PriceMapping>::new)
            .push(price_mapping);
        self
    }

    #[inline]
    pub fn len(&self) -> usize { self.map.len() }
}
