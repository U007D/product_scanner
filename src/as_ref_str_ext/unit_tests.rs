use super::*;

#[test]
fn as_product_list_with_valid_chars_yields_product_list() {
    // given a `str` with only valid products
    let prods = "aBcDdCbA";

    // when converted to a product list
    let result = prods.as_product_list();

    // then it should succeed with the expected result
    assert_eq!(result, Ok(vec![Product::A, Product::B, Product::C, Product::D,
                               Product::D, Product::C, Product::B, Product::A]));
}

#[test]
fn as_product_list_with_invalid_char_yields_error() {
    // given a `str` containing an invalid product mnemonic
    let prods = "ABECD";

    // when converted to a product list
    let result = prods.as_product_list();

    // then the expected error should be returned
    assert_eq!(result, Err(Error::InvalidProductMnemonic('E')));
}
