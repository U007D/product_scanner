//! Problem statement
//!
//! Consider a store where items have prices per unit but also volume prices. For example, apples may be $1.00 each or
//! 4 for $3.00. Implement a point-of-sale scanning API that accepts an arbitrary ordering of products (similar to what
//! would happen at a checkout line) and then returns the correct total price for an entire shopping cart based on the
//! per unit prices or the volume prices as applicable.
//!
//! Here are the products listed by code and the prices to use (there is no sales tax):
//!
//! Ensure that the assignment is submitted with working test cases
//! Product Code 	Price
//! A 	$2 each or 4 for $7
//! B 	$12
//! C 	$1.25 each or $6 for a six-pack
//! D 	$.15
//!
//! There should be a top level point of sale terminal service object that looks something like the pseudo-code below.
//! You are free to design and implement the rest of the code however you wish, including how you specify the prices in
//! the system:
//!
//! terminal.setPricing(...)
//! terminal.scan("A")
//! terminal.scan("C")
//! ... etc.
//! result = terminal.total
//!
//! Here are the minimal inputs you should use for your test cases. These test cases must be shown to work in your
//!     program:
//!     Scan these items in this order: ABCDABAA; Verify the total price is $32.40.
//!     Scan these items in this order: CCCCCCC; Verify the total price is $7.25.
//!     Scan these items in this order: ABCD; Verify the total price is $15.40.
//!
//! General Instructions:-
//!
//! - Don't use any mutable 'vars', nulls or throw Exceptions - (hint: use vals, Options and Eithers instead, if you need them).
//! - Bonus points: demonstrate use of a type parameter and/or a higher order function.

#![warn(clippy::all)]
#![forbid(unsafe_code)] // Do not remove!  Explicitly change to #![allow(unsafe_code)] to use `unsafe` keyword.
#![forbid(overflowing_literals)]
// Uncomment before ship to reconcile use of possibly redundant crates and uncover possible debug remnants
#![warn(clippy::multiple_crate_versions, clippy::print_stdout, clippy::unimplemented, clippy::use_debug)]
#![deny(warnings)]
#![deny(missing_docs)]
// vvv Safety-critical application lints vvv
#![deny(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_precision_loss,
clippy::cast_sign_loss, clippy::float_cmp_const, clippy::indexing_slicing, /* clippy::integer_arithmetic, */
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

mod args;
mod as_ref_str_ext;
mod consts;
mod error;
mod ord_decimal;
mod product;
mod price_list;
mod quantity_price;
mod terminal;

/// As a convenience, make `Result<T>` equivalent to `std::result::Result<T, crate::error::Error>`.
pub type Result<T> = StdResult<T, Error>;

fn main() -> Result<()> {
    let _args = Args::from_args();
    Ok(())
}

