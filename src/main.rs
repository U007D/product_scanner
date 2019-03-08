#![warn(clippy::all)]
#![forbid(unsafe_code)] // Do not remove!  Explicitly change to #![allow(unsafe_code)] to use `unsafe` keyword.
#![forbid(overflowing_literals)]
//#![deny(warnings)]
//#![deny(missing_docs)]
// Uncomment before ship to reconcile use of possibly redundant crates and uncover possible debug remnants
//#![warn(clippy::multiple_crate_versions, clippy::print_stdout, clippy::unimplemented, clippy::use_debug)]
// vvv Safety-critical application lints (pedantic: use for safety-critical applications only) vvv
#![deny(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_precision_loss,
clippy::cast_sign_loss, clippy::float_cmp_const, clippy::indexing_slicing, clippy::integer_arithmetic,
clippy::maybe_infinite_iter, clippy::option_unwrap_used, clippy::result_unwrap_used)]
// ^^^ End of safety-critical lint section ^^^
#![allow(clippy::match_bool,)]

use std::result::Result as StdResult;

use structopt::StructOpt;

pub use {
    args::Args,
    consts::*,
    error::Error,
    ord_decimal::OrdDecimal as Decimal,
    price_list::PriceListBuilder,
    product::Product,
    terminal::Terminal,
};
use op::Op;

mod args;
mod consts;
mod error;
mod op;
mod ord_decimal;
mod product;
mod price_list;
mod price_mapping;
mod terminal;

pub type Result<T> = StdResult<T, Error>;

fn main() -> Result<()> {
    let _args = Args::from_args();
    Ok(())
}

