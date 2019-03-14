use std::collections::{BTreeSet, HashMap};

use crate::{
    Error,
    price::Price,
    Product,
    Result,
};

#[cfg(test)]
mod unit_tests;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PriceList {
    entries: HashMap<Product, BTreeSet<Price>>,
}

impl PriceList {
    pub fn new() -> Self {
        Self {
            entries: HashMap::<Product, BTreeSet<Price>>::new()
        }
    }

    fn add_entry(&mut self, product: Product, price_mapping: Price) -> &mut Self {
        self.entries.entry(product)
            .or_insert_with(BTreeSet::<Price>::new)
            .insert(price_mapping);
        self
    }

    // TODO: change return type to `impl Trait` (loose coupling)
    pub fn find_product_pricing(&self, product: Product) -> Option<&BTreeSet<Price>> {
        self.entries.get(&product)
    }

    #[inline]
    pub fn len(&self) -> usize { self.entries.len() }
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

    pub fn set_pricing(mut self, product: Product, price_mapping: Price) -> Self {
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

