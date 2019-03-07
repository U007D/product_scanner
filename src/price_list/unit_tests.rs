#![allow(clippy::option_unwrap_used, clippy::result_unwrap_used)]
use super::*;
use crate::price_mapping::PriceMapping;
use ordered_float::NotNan;
use std::num::NonZeroU32;

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
fn build_after_adding_valid_product_pricing_yields_valid_price_list() {
    // environment
    let (prod, price, quantity) = (Product::A, NotNan::new(0.99).unwrap(), NonZeroU32::new(1).unwrap());
    let mapping = PriceMapping::new(price, quantity).unwrap();

    // given a price list builder with a valid price list
    let mut builder = PriceListBuilder::new();
    builder.set_pricing(prod, mapping.clone());

    // when adding a valid product and pricing entry before building
    let result = builder.build();

    // then it should yield a PriceList with exactly the expected number of price mappings
    let result = result.unwrap();
    assert_eq!(result.len(), 1);

    // and the price mapping should contain the correct values
    #[allow(clippy::indexing_slicing, clippy::float_cmp)]
    {
        assert_eq!(result.prices[&prod].get(&mapping).is_some(), true);
    }
}
