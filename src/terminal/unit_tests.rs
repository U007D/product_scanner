use crate::{
    Decimal,
    price_mapping::PriceMapping,
    PriceListBuilder,
    Product
};

use super::*;

#[test]
fn scan_valid_product_yields_correct_total() {
    // given a valid terminal
    let price_list = PriceListBuilder::new()
                                      .set_pricing(Product::A, PriceMapping::new(Decimal::from(4.2), 1).unwrap())
                                      .build()
                                      .unwrap();
    let terminal = Terminal::new(price_list);

    // when a valid product is scanned
    let result = terminal.scan([Product::A]);

    // then the correct total should be returned
    assert_eq!(result, Decimal::from(4.2));
}
