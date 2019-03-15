#![allow(clippy::option_unwrap_used, clippy::result_unwrap_used)]
use crate::{
    as_ref_str_ext::AsRefStrExt,
    Decimal,
    quantity_price::QuantityPrice,
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
            .add_price_list_entry(Product::A, QuantityPrice::new(Decimal::from(4.2), NonZeroUsize::new(1).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list);

    // when a valid product is scanned
    let result = terminal.scan(&"A".as_product_list().unwrap());

    // then the correct total should be returned
    assert_eq!(Ok(Decimal::from(4.2)), result);
}

#[test]
fn scan_one_invalid_product_yields_error() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .add_price_list_entry(Product::A, QuantityPrice::new(Decimal::from(4.2), NonZeroUsize::new(1).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list.clone());

    // when an invalid product is scanned (`price_list` only contains pricing data for `Product::A`)
    let result = terminal.scan(&"B".as_product_list().unwrap());

    // then the expected `Error` should result
    assert_eq!(Err(Error::ProductNotFound(Product::B, price_list)), result);
}

#[test]
fn scan_invalid_quantity_of_valid_product_yields_error() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .add_price_list_entry(Product::A, QuantityPrice::new(Decimal::from(4.2), NonZeroUsize::new(2).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list.clone());

    // when an invalid quantity of valid product is scanned (`price_list` does not contain entry for quantity 1)
    let result = terminal.scan(&"A".as_product_list().unwrap());

    // then the expected `Error` should result
    assert_eq!(Err(Error::PricingNotFoundAtQuantity(Product::A, NonZeroUsize::new(1).unwrap(), price_list)), result);
}

#[test]
fn scan_three_valid_products_yields_correct_total() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .add_price_list_entry(Product::A, QuantityPrice::new(Decimal::from(4.2), NonZeroUsize::new(1).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list);

    // when a valid product is scanned
    let result = terminal.scan(&"AAA".as_product_list().unwrap());

    // then the correct total should be returned
    assert_eq!(Ok(Decimal::from(12.6)), result);
}

#[test]
fn scan_mix_of_valid_products_yields_correct_total() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .add_price_list_entry(Product::A, QuantityPrice::new(Decimal::from(4.2), NonZeroUsize::new(1).unwrap()).unwrap())
            .add_price_list_entry(Product::B, QuantityPrice::new(Decimal::from(0.5), NonZeroUsize::new(1).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list);

    // when a valid product is scanned
    let result = terminal.scan(&"AABA".as_product_list().unwrap());

    // then the correct total should be returned
    assert_eq!(Ok(Decimal::from(13.1)), result);
}

#[test]
fn scan_mix_of_valid_and_invalid_product_yields_error() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .add_price_list_entry(Product::A, QuantityPrice::new(Decimal::from(4.2), NonZeroUsize::new(1).unwrap()).unwrap())
            .add_price_list_entry(Product::B, QuantityPrice::new(Decimal::from(0.5), NonZeroUsize::new(1).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list.clone());

    // when an invalid product is scanned (`price_list` does not contain pricing data for `Product::C`)
    let result = terminal.scan(&"AABCA".as_product_list().unwrap());

    // then the expected `Error` should result
    assert_eq!(Err(Error::ProductNotFound(Product::C, price_list)), result);
}

#[test]
fn scan_multiple_valid_discounted_product_yields_correct_total() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .add_price_list_entry(Product::A, QuantityPrice::new(Decimal::from(4.2), NonZeroUsize::new(1).unwrap()).unwrap())
            .add_price_list_entry(Product::A, QuantityPrice::new(Decimal::from(7), NonZeroUsize::new(2).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list);

    // when a valid product is scanned
    let result = terminal.scan(&"AA".as_product_list().unwrap());

    // then the correct total should be returned
    assert_eq!(Ok(Decimal::from(7)), result);
}

#[test]
fn scan_test_case_1_yields_proper_total() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .add_price_list_entry(Product::A, QuantityPrice::new(Decimal::from(2), NonZeroUsize::new(1).unwrap()).unwrap())
            .add_price_list_entry(Product::A, QuantityPrice::new(Decimal::from(7), NonZeroUsize::new(4).unwrap()).unwrap())
            .add_price_list_entry(Product::B, QuantityPrice::new(Decimal::from(12), NonZeroUsize::new(1).unwrap()).unwrap())
            .add_price_list_entry(Product::C, QuantityPrice::new(Decimal::from(1.25), NonZeroUsize::new(1).unwrap()).unwrap())
            .add_price_list_entry(Product::C, QuantityPrice::new(Decimal::from(6), NonZeroUsize::new(6).unwrap()).unwrap())
            .add_price_list_entry(Product::D, QuantityPrice::new(Decimal::from(0.15), NonZeroUsize::new(1).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list);

    // when a valid product is scanned
    let result = terminal.scan(&"ABCDABAA".as_product_list().unwrap());

    // then the correct total should be returned
    assert_eq!(Ok(Decimal::from(32.4)), result);
}

#[test]
fn scan_test_case_2_yields_proper_total() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .add_price_list_entry(Product::A, QuantityPrice::new(Decimal::from(2), NonZeroUsize::new(1).unwrap()).unwrap())
            .add_price_list_entry(Product::A, QuantityPrice::new(Decimal::from(7), NonZeroUsize::new(4).unwrap()).unwrap())
            .add_price_list_entry(Product::B, QuantityPrice::new(Decimal::from(12), NonZeroUsize::new(1).unwrap()).unwrap())
            .add_price_list_entry(Product::C, QuantityPrice::new(Decimal::from(1.25), NonZeroUsize::new(1).unwrap()).unwrap())
            .add_price_list_entry(Product::C, QuantityPrice::new(Decimal::from(6), NonZeroUsize::new(6).unwrap()).unwrap())
            .add_price_list_entry(Product::D, QuantityPrice::new(Decimal::from(0.15), NonZeroUsize::new(1).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list);

    // when a valid product is scanned
    let result = terminal.scan(&"CCCCCCC".as_product_list().unwrap());

    // then the correct total should be returned
    assert_eq!(Ok(Decimal::from(7.25)), result);
}

#[test]
fn scan_test_case_3_yields_proper_total() {
    // given a valid terminal
    let price_list =
        PriceListBuilder::new()
            .add_price_list_entry(Product::A, QuantityPrice::new(Decimal::from(2), NonZeroUsize::new(1).unwrap()).unwrap())
            .add_price_list_entry(Product::A, QuantityPrice::new(Decimal::from(7), NonZeroUsize::new(4).unwrap()).unwrap())
            .add_price_list_entry(Product::B, QuantityPrice::new(Decimal::from(12), NonZeroUsize::new(1).unwrap()).unwrap())
            .add_price_list_entry(Product::C, QuantityPrice::new(Decimal::from(1.25), NonZeroUsize::new(1).unwrap()).unwrap())
            .add_price_list_entry(Product::C, QuantityPrice::new(Decimal::from(6), NonZeroUsize::new(6).unwrap()).unwrap())
            .add_price_list_entry(Product::D, QuantityPrice::new(Decimal::from(0.15), NonZeroUsize::new(1).unwrap()).unwrap())
            .build()
            .unwrap();

    let terminal = Terminal::new(price_list);

    // when a valid product is scanned
    let result = terminal.scan(&"ABCD".as_product_list().unwrap());

    // then the correct total should be returned
    assert_eq!(Ok(Decimal::from(15.40)), result);
}
