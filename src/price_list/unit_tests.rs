#![allow(clippy::option_unwrap_used, clippy::result_unwrap_used, clippy::indexing_slicing)]

use std::num::NonZeroU32;

use crate::{
    Decimal,
    price_mapping::PriceMapping
};

use super::*;

#[test]
fn build_empty_product_list_fails() {
    // given a price list builder
    let builder = PriceListBuilder::new();

    // when an empty price list is build
    let result = builder.build();

    // then it should fail when invoked, returning the expected error
    assert_eq!(result, Err(Error::EmptyProductPricingTable));
}

#[test]
fn set_pricing_adding_valid_product_pricing_yields_valid_price_list() {
    // environment
    let (prod, price, quantity) = (Product::A, Decimal::from(0.99), NonZeroU32::new(1).unwrap());
    let mapping = PriceMapping::new(price.clone(), quantity).unwrap();
    let non_existent_mapping = PriceMapping::new(price, NonZeroU32::new(2).unwrap()).unwrap();

    // given a price list builder
    let builder = PriceListBuilder::new();

    // when adding a valid product and pricing entry before building
    let result = builder.set_pricing(prod, mapping.clone())
                        .build();

    // then it should yield a PriceList with exactly the expected number of price mappings
    let result = result.unwrap();
    assert_eq!(result.len(), 1);

    // and the price mapping should contain the correct values
    assert_eq!(result.entries[&prod].get(&non_existent_mapping).is_none(), true);
    assert_eq!(result.entries[&prod].get(&mapping).is_some(), true);
}
