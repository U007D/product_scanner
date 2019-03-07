use crate::{
    consts::msg,
    Product
};
use derive_more::*;
use std::num::NonZeroU32;
use crate::price_mapping::PriceMapping;

#[derive(Clone, Debug, Display, From, PartialEq)]
pub enum Error {
    #[display(fmt = "{}: {:?}", "msg::ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8", "_0")]
    ArgNotConvertibleToUtf8(std::ffi::OsString),
    #[display(fmt = "{}", "msg::ERR_EMPTY_PRODUCT_PRICING_TABLE")]
    EmptyProductPricingTable,
    #[display(fmt = "{}: {:?}, {:?}", "msg::ERR_INTERNAL_KVP_ALREADY_PRESENT", "_0", "_1")]
    KvpAlreadyPresent(Product, Vec<PriceMapping>),
    #[display(fmt = "{}: {}/{}", "msg::ERR_UNIT_PRICE_NOT_REPRESENTABLE", "_0", "_1")]
    UnitPriceNotRepresentable(f64, NonZeroU32),
}
