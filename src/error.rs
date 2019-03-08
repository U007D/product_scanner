use derive_more::{
    Display,
    From,
};

use crate::{
    consts::msg,
    Decimal,
    Op,
    price_list::PriceList,
    price_mapping::PriceMapping,
    Product
};

#[derive(Clone, Debug, Display, From, PartialEq)]
pub enum Error {
    #[display(fmt = "{}: {:?}", "msg::ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8", "_0")]
    ArgNotConvertibleToUtf8(std::ffi::OsString),
    #[display(fmt = "{}", "msg::ERR_EMPTY_PRODUCT_PRICING_TABLE")]
    EmptyProductPricingTable,
    #[display(fmt = "{}: {:?}, {:?}", "msg::ERR_INTERNAL_KVP_ALREADY_PRESENT", "_0", "_1")]
    KvpAlreadyPresent(Product, Vec<PriceMapping>),
    #[display(fmt = "{}: {:?}", "msg::ERR_UNREPRESENTABLE_VALUE", "_0")]
    OpYieldedUnrepresentableValue(Op<Decimal>),
    #[display(fmt = "{}: {:?}, {:?}", "msg::ERR_PRODUCT_NOT_FOUND", "_0", "_1")]
    ProductNotFound(Product, PriceList),
}
