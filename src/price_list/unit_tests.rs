#![allow(clippy::option_unwrap_used, clippy::result_unwrap_used, clippy::indexing_slicing)]

use std::num::NonZeroUsize;

use crate::{
    Decimal,
    quantity_price::QuantityPrice
};

use super::*;

#[test]
fn build_empty_product_list_fails() {
    // given a price list builder
    let builder = PriceListBuilder::new();

    // when an empty price list is build
    let result = builder.build();

    // then it should fail when invoked, returning the expected error
    assert_eq!(Err(Error::EmptyProductPricingTable), result);
}

#[test]
fn set_pricing_adding_valid_product_pricing_yields_valid_price_list() {
    // environment
    let (prod, price, quantity) = (Product::A, Decimal::from(0.99), NonZeroUsize::new(1).unwrap());
    let mapping = QuantityPrice::new(price.clone(), quantity).unwrap();
    let non_existent_mapping = QuantityPrice::new(price, NonZeroUsize::new(2).unwrap()).unwrap();

    // given a price list builder
    let builder = PriceListBuilder::new();

    // when adding a valid product and pricing entry before building
    let result = builder.add_price_list_entry(prod, mapping.clone())
                        .build();

    // then it should yield a PriceList with exactly the expected number of price mappings
    let result = result.unwrap();
    assert_eq!(1, result.len());

    // and the price mapping should contain the correct values
    assert_eq!(true, result.entries[&prod].get(&non_existent_mapping).is_none());
    assert_eq!(true, result.entries[&prod].get(&mapping).is_some());
}
