use super::*;
use rspec::{
    given,
    run,
};
use crate::pricing::PriceList;
use std::num::NonZeroU32;

#[derive(Clone, Debug)]
struct Env {
    builder: PriceListBuilder,
    product_pricing: Option<Result<PriceList>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            builder: PriceListBuilder::new(),
            product_pricing: None,
        }
    }
}

#[test]
fn rspec() {
    run(&given("a product pricing builder", Env::new(), |ctx| {
        ctx.when("nothing is added to the builder", |ctx| {
            ctx.before_each(|env| env.product_pricing = Some(env.builder.build()));

            ctx.then("it should fail when built, returning the expected error", |env| {
                env.product_pricing == Some(Err(Error::EmptyProductPricingTable))
            })
        });

        ctx.when("a valid product and pricing entry is added", |ctx| {
            ctx.before_each(|env| env.product_pricing = Some(env.builder
                                                                .set_pricing(Product::A, 0.99, NonZeroU32::new(1)
                                                                                                          .unwrap())
                                                                .unwrap()
                                                                .build()));

            ctx.then("it should succeed", |env| env.product_pricing.unwrap().is_ok());
//            ctx.then("it should contain only the expected product",
//                     |env| env.product_pricing
//                              .unwrap()
//                              .unwrap()
//                              .find(""))
        })
    }))
}
//#[test]
//fn new_empty_product_pricing_table_fails() {
//    // given a product pricing builder
//    let builder = ProductPricingBuilder::new();
//
//    // when nothing is added to the builder
//
//    // then it should fail when invoked, returning the expected error
//    assert_eq!(builder.build(), Err(Error::EmptyProductPricingTable));
//}
//
//#[test]
//fn add_product_pricing_yields_pricing_table_with_item_and_correct_pricing() {
//    // given a product pricing builder
//    let builder = ProductPricingBuilder::new();
//
//    // when a product price mapping as added to the builder
//    let result = builder.build();
//
//    // then it should fail return the expected error
//    assert_eq!(result.build, Err(Error::EmptyProductPricingTable));
//}
