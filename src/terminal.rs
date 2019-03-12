use std::{
    borrow::Borrow,
    collections::HashMap,
    num::NonZeroUsize,
};

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

    #[allow(clippy::integer_arithmetic)]
    fn calc_line_total(&self, prod_quant: (Product, usize)) -> Result<Decimal> {
        let (prod, quant) = prod_quant;
        self.price_list
            .find_product_pricing(prod)
            .ok_or_else(|| Error::ProductNotFound(prod, self.price_list.clone()))
            .and_then(|prices|
                  prices.iter()
                      .scan(quant, |quant, price_map|{
                          *quant %= price_map.quantity.get();
                          Some((*quant / price_map.quantity.get(), price_map.price.clone(), price_map.quantity.get()))
                      })
                      .fold((Err(Error::PricingNotFoundAtQuantity(
                                prod,
                                NonZeroUsize::new(quant).expect(msg::ERR_INTERNAL_ZERO_USED_WITH_NON_ZERO_TYPE),
                                self.price_list.clone())),
                            Decimal::from(0)),
                            |res, item| {
                                match item.0 == 0 {
                                    true => res,
                                    false => {
                                        let tot = res.1 + Decimal::from(item.0) * item.1;
                                        (Ok(tot.clone()), tot)
                                    }
                                }
                            })
                      .0)
//                prices.iter()
//                      .find(|price_map| price_map.quantity.get() <= quant)
//                      .ok_or_else(||
//                          Error::PricingNotFoundAtQuantity(
//                              prod,
//                              NonZeroUsize::new(quant).expect(msg::ERR_INTERNAL_ZERO_USED_WITH_NON_ZERO_TYPE),
//                              self.price_list.clone()))
//                      .and_then(|price_map| Ok(price_map.unit_price.clone() * Decimal::from(quant))))
    }

    fn product_quantities<I>(&self, products: I) -> Result<impl Iterator<Item = (Product, usize)>>
                             where I: IntoIterator,
                                   I::Item: Borrow<Product>, {
        products.into_iter()
                .try_fold(HashMap::<Product, usize>::new(),
                          |mut prod_quants, prod| {
                              let key = prod.borrow();
                              match prod_quants.get_mut(key) {
                                  Some(val) => {
                                      val.checked_add(1)
                                         .ok_or_else(|| Error::IntegerOverflow(Op::Add(*val, 1)))
                                         .and_then(|v| {
                                             *val = v;
                                             Ok(())
                                         })
                                  },
                                  None => {
                                      prod_quants.insert(*key, 1);
                                      Ok(())
                                  },
                              }
                              .and_then(|_| Ok(prod_quants))
                          })
                .and_then(|q_prods| Ok(q_prods.into_iter()))
    }

    pub fn scan<I>(&self, products: I) -> Result<Decimal> where I: IntoIterator,
                                                                I::Item: Borrow<Product>, {
        self.product_quantities(products)?
            .try_fold(Decimal::from(0),
                      |acc, prod_quant| self.calc_line_total(prod_quant)
                                           .and_then(|line_total|
                                               acc.checked_add(&line_total)
                                                  .ok_or_else(|| Error::OpYieldedInvalidDecimalValue(
                                                      Op::Add(acc, line_total)))))
    }
}
