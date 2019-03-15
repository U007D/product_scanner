use std::num::NonZeroUsize;

use derive_more::{
    Display,
    From,
};

use crate::{
    consts::msg,
    price_list::PriceList,
    quantity_price::QuantityPrice,
    Product
};

/// `PriceScanner` error type.
#[derive(Clone, Debug, Display, From, PartialEq)]
pub enum Error {
    /// Occurs when the system cannot convert an [OsString] to a UTF-8 string.
    #[display(fmt = "{}: {:?}", "msg::ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8", "_0")]
    ArgNotConvertibleToUtf8(std::ffi::OsString),

    /// Occurs when the [PriceListBuilder] is asked to build without having been loaded with pricing data.
    #[display(fmt = "{}", "msg::ERR_EMPTY_PRODUCT_PRICING_TABLE")]
    EmptyProductPricingTable,

    /// Should to occur--indicative of an implementation bug: occurs when upon checking for the existence of a key,
    /// while exclusively locking access, the key being added miraculously becomes 'already present'.
    #[display(fmt = "{}: {:?}, {:?}", "msg::ERR_INTERNAL_KVP_ALREADY_PRESENT", "_0", "_1")]
    KvpAlreadyPresent(Product, Vec<QuantityPrice>),

    /// Occurs when a valid [Product] is not present in the `PriceList`
    #[display(fmt = "{}: {:?}, {:?}", "msg::ERR_PRODUCT_NOT_FOUND", "_0", "_1")]
    ProductNotFound(Product, PriceList),

    /// Occurs when the quantity of [Product] being purchased is below the quantity for every entry of that product in
    /// the [PriceList].  To ensure this does not occur, ensure every product has at least a quantity 1 offering in the
    /// [PriceList].
    #[display(fmt = "{}: {:?}, {:?}, {:?}", "msg::ERR_PRICING_NOT_FOUND_AT_QUANTITY", "_0", "_1", "_2")]
    PricingNotFoundAtQuantity(Product, NonZeroUsize, PriceList),

    /// Occurs when [AsRefStrExt::as_product_list()] helper function is provided with input which does not match any
    /// existing [Product].
    #[display(fmt = "{}: {}", "msg::ERR_INVALID_PRODUCT_MNEMONIC", "_0")]
    InvalidProductMnemonic(char),
}
