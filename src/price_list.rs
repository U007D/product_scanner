use std::collections::{BTreeSet, HashMap};

use crate::{
    Error,
    quantity_price::QuantityPrice,
    Product,
    Result,
};

#[cfg(test)]
mod unit_tests;

/// `PriceList` represents price mappings of all available products at (effectively) unlimited levels of quantity
/// pricing.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PriceList {
    entries: HashMap<Product, BTreeSet<QuantityPrice>>,
}

impl PriceList {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            entries: HashMap::<Product, BTreeSet<QuantityPrice>>::new()
        }
    }

    /// Add a `Product`-> Price @ Quantity mapping to the `PriceList`.  Regardless of the order in which the entries are
    /// added, `PriceList` is organized such that `Terminal::scan()`s always yield the lowest possible price to the
    /// consumer.
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

/// `PriceListBuilder` employs the [Builder Pattern](https://en.wikipedia.org/wiki/Builder_pattern) to ensure all
/// `PriceLists` are immutable.  The Builder Pattern also provides the opportunity to inject business logic into the
/// `PriceList` without impacting performance after initialization.  Note that this pattern also facilitates generating
/// custom price lists--identified frequent customers could `scan` their purchases against a discounted list,
/// for example.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PriceListBuilder {
    price_list: PriceList,
}

impl PriceListBuilder {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            price_list: PriceList::default(),
        }
    }

    /// Adds an entry to be included in the built `PriceList`.  Does not enforce unique price/quantity mappings
    /// duplicates have no effect.  Generated
    pub fn add_price_list_entry(mut self, product: Product, quant_price: QuantityPrice) -> Self {
        self.price_list.add_entry(product, quant_price);
        self
    }

    /// Construct the `PriceList`.
    pub fn build(self) -> Result<PriceList> {
        match self.price_list.len() {
            0 => Err(Error::EmptyProductPricingTable),
            _ => Ok(self.price_list),
        }
    }
}

