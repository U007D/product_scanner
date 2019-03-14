use std::collections::{BTreeSet, HashMap};

use crate::{
    Error,
    quantity_price::QuantityPrice,
    Product,
    Result,
};

#[cfg(test)]
mod unit_tests;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PriceList {
    entries: HashMap<Product, BTreeSet<QuantityPrice>>,
}

impl PriceList {
    pub fn new() -> Self {
        Self {
            entries: HashMap::<Product, BTreeSet<QuantityPrice>>::new()
        }
    }

    fn add_entry(&mut self, product: Product, price_mapping: QuantityPrice) -> &mut Self {
        self.entries.entry(product)
            .or_insert_with(BTreeSet::<QuantityPrice>::new)
            .insert(price_mapping);
        self
    }

    // TODO: change return type to `impl Trait` (loose coupling)
    pub fn product_price_list(&self, product: Product) -> Option<&BTreeSet<QuantityPrice>> {
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

    pub fn add_product_price_list_entry(mut self, product: Product, quant_price: QuantityPrice) -> Self {
        self.price_list.add_entry(product, quant_price);
        self
    }

    pub fn build(self) -> Result<PriceList> {
        match self.price_list.len() {
            0 => Err(Error::EmptyProductPricingTable),
            _ => Ok(self.price_list),
        }
    }
}

