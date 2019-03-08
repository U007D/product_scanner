use std::borrow::Borrow;
use std::num::NonZeroU32;

use fraction::CheckedAdd;

use crate::{
    Decimal,
    Error,
    msg,
    op::Op,
    price_list::PriceList,
    Product,
    Result
};

#[cfg(test)]
mod unit_tests;

pub struct Terminal {
    price_list: PriceList,
}

impl Terminal {
    pub fn new(price_list: PriceList) -> Self {
        Self {
            price_list,
        }
    }

    pub fn scan<I>(&self, products: I) -> Result<Decimal> where I: IntoIterator,
                                                                I::Item: Borrow<Product>, {
        // TODO: compute quantities
        products.into_iter()
                .try_fold(
                    Decimal::from(0),
                    |acc, prod|
                        // find product
                        self.price_list
                            .find_product_pricing(*prod.borrow())
                            .ok_or_else(|| Error::ProductNotFound(*prod.borrow(), self.price_list.clone()))
                            .and_then(|prices|
                                prices.iter()
                                      .find(|m|
                                          m.quantity <= NonZeroU32::new(1)
                                                                   .expect(msg::ERR_INTERNAL_INFALLIBLE_VALID_CONSTANT))
                                      .ok_or_else(||
                                          Error::PricingNotFoundAtQuantity(
                                              *prod.borrow(),
                                              NonZeroU32::new(1)
                                                         .expect(msg::ERR_INTERNAL_INFALLIBLE_VALID_CONSTANT),
                                              self.price_list.clone())))
                            // tally pricing
                            .and_then(|price| acc.checked_add(&price.price)
                                                 .ok_or_else(||
                                                     Error::OpYieldedUnrepresentableValue(Op::Add(acc,
                                                                                                  price.price.clone())))
                            ))
    }
}
