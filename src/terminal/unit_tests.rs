#![allow(clippy::option_unwrap_used, clippy::result_unwrap_used)]
use crate::{
    Decimal,
    price_mapping::PriceMapping,
    PriceListBuilder,
    Product
};
use std::num::NonZeroUsize;
use super::*;

#[test]
fn scan_one_valid_product_yields_correct_total() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .set_pricing(Product::A, PriceMapping::new(Decimal::from(4.2), NonZeroUsize::new(1).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list);

    // when a valid product is scanned
    let result = terminal.scan(&[Product::A]);

    // then the correct total should be returned
    assert_eq!(result, Ok(Decimal::from(4.2)));
}

#[test]
fn scan_one_invalid_product_yields_error() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .set_pricing(Product::A, PriceMapping::new(Decimal::from(4.2), NonZeroUsize::new(1).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list.clone());

    // when an invalid product is scanned (`price_list` only contains pricing data for `Product::A`)
    let result = terminal.scan(&[Product::B]);

    // then the expected `Error` should result
    assert_eq!(result, Err(Error::ProductNotFound(Product::B, price_list)));
}

#[test]
fn scan_invalid_quantity_of_valid_product_yields_error() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .set_pricing(Product::A, PriceMapping::new(Decimal::from(4.2), NonZeroUsize::new(2).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list.clone());

    // when an invalid quantity of valid product is scanned (`price_list` does not contain entry for quantity 1)
    let result = terminal.scan(&[Product::A]);

    // then the expected `Error` should result
    assert_eq!(result, Err(Error::PricingNotFoundAtQuantity(Product::A, NonZeroUsize::new(1).unwrap(), price_list)));
}

#[test]
fn scan_three_valid_products_yields_correct_total() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .set_pricing(Product::A, PriceMapping::new(Decimal::from(4.2), NonZeroUsize::new(1).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list);

    // when a valid product is scanned
    let result = terminal.scan(&[Product::A, Product::A, Product::A]);

    // then the correct total should be returned
    assert_eq!(result, Ok(Decimal::from(12.6)));
}

#[test]
fn scan_mix_of_valid_products_yields_correct_total() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .set_pricing(Product::A, PriceMapping::new(Decimal::from(4.2), NonZeroUsize::new(1).unwrap()).unwrap())
            .set_pricing(Product::B, PriceMapping::new(Decimal::from(0.5), NonZeroUsize::new(1).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list);

    // when a valid product is scanned
    let result = terminal.scan(&[Product::A, Product::A, Product::B, Product::A]);

    // then the correct total should be returned
    assert_eq!(result, Ok(Decimal::from(13.1)));
}

#[test]
fn scan_mix_of_valid_and_invalid_product_yields_error() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .set_pricing(Product::A, PriceMapping::new(Decimal::from(4.2), NonZeroUsize::new(1).unwrap()).unwrap())
            .set_pricing(Product::B, PriceMapping::new(Decimal::from(0.5), NonZeroUsize::new(1).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list.clone());

    // when an invalid product is scanned (`price_list` does not contain pricing data for `Product::C`)
    let result = terminal.scan(&[Product::A, Product::A, Product::B, Product::C, Product::A]);

    // then the expected `Error` should result
    assert_eq!(result, Err(Error::ProductNotFound(Product::C, price_list)));
}

#[test]
fn scan_multiple_valid_discounted_product_yields_correct_total() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .set_pricing(Product::A, PriceMapping::new(Decimal::from(4.2), NonZeroUsize::new(1).unwrap()).unwrap())
            .set_pricing(Product::A, PriceMapping::new(Decimal::from(7), NonZeroUsize::new(2).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list);

    // when a valid product is scanned
    let result = terminal.scan(&[Product::A, Product::A]);

    // then the correct total should be returned
    assert_eq!(result, Ok(Decimal::from(7)));
}

#[test]
fn scan_test_case_1_yields_proper_total() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .set_pricing(Product::A, PriceMapping::new(Decimal::from(2), NonZeroUsize::new(1).unwrap()).unwrap())
            .set_pricing(Product::A, PriceMapping::new(Decimal::from(7), NonZeroUsize::new(4).unwrap()).unwrap())
            .set_pricing(Product::B, PriceMapping::new(Decimal::from(12), NonZeroUsize::new(1).unwrap()).unwrap())
            .set_pricing(Product::C, PriceMapping::new(Decimal::from(1.25), NonZeroUsize::new(1).unwrap()).unwrap())
            .set_pricing(Product::C, PriceMapping::new(Decimal::from(6), NonZeroUsize::new(6).unwrap()).unwrap())
            .set_pricing(Product::D, PriceMapping::new(Decimal::from(0.15), NonZeroUsize::new(1).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list);

    // when a valid product is scanned
    let result = terminal.scan(&[Product::A, Product::B, Product::C, Product::D,
        Product::A, Product::B, Product::A, Product::A]);

    // then the correct total should be returned
    assert_eq!(result, Ok(Decimal::from(32.4)));
}

#[test]
fn scan_test_case_2_yields_proper_total() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .set_pricing(Product::A, PriceMapping::new(Decimal::from(2), NonZeroUsize::new(1).unwrap()).unwrap())
            .set_pricing(Product::A, PriceMapping::new(Decimal::from(7), NonZeroUsize::new(4).unwrap()).unwrap())
            .set_pricing(Product::B, PriceMapping::new(Decimal::from(12), NonZeroUsize::new(1).unwrap()).unwrap())
            .set_pricing(Product::C, PriceMapping::new(Decimal::from(1.25), NonZeroUsize::new(1).unwrap()).unwrap())
            .set_pricing(Product::C, PriceMapping::new(Decimal::from(6), NonZeroUsize::new(6).unwrap()).unwrap())
            .set_pricing(Product::D, PriceMapping::new(Decimal::from(0.15), NonZeroUsize::new(1).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list);

    // when a valid product is scanned
    let result = terminal.scan(&[Product::C, Product::C, Product::C, Product::C,
                                 Product::C, Product::C, Product::C]);

    // then the correct total should be returned
    assert_eq!(result, Ok(Decimal::from(7.25)));
}

