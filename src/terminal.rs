use std::{
    borrow::Borrow,
    collections::HashMap,
};

use fraction::{
    CheckedAdd,
    CheckedMul,
};

use crate::{
    Decimal,
    Error,
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

    fn calc_line_total(&self, prod: Product, quant: usize) -> Result<Decimal> {
        self.price_list
            .find_product_pricing(prod)
            .ok_or_else(|| Error::ProductNotFound(prod, self.price_list.clone()))
            .and_then(|prod_price_list|
                prod_price_list.iter()
                               .scan(quant, |remaining_quant, price_map| {
                                   let price_map_quant = price_map.quantity.get();
                                   match dbg!(&remaining_quant).checked_div(dbg!(price_map_quant)) {
                                       Some(q) if q > 0 => {
                                           Some(remaining_quant.checked_rem(price_map_quant)
                                                               .ok_or_else(||
                                                                   Error::IntegerOverflow(Op::Rem(*remaining_quant,
                                                                                                  price_map_quant)))
                                                               .and_then(|rq| {
                                                                   *remaining_quant = rq;
                                                                   Decimal::from(q)
                                                                       .checked_mul(&price_map.price.clone())
                                                                       .ok_or_else(||
                                                                           Error::OpYieldedInvalidDecimalValue(
                                                                               Op::Mul(Decimal::from(q),
                                                                                       price_map.price.clone())))
                                                               }))
                                       },
                                       Some(_) => Some(Ok(Decimal::from(0))),
                                       None => Some(Err(Error::IntegerOverflow(Op::Div(*remaining_quant,
                                                                                       price_map_quant)))),
                                   }
                               })
                               .try_fold(Decimal::from(0),
                                         |acc, line_price| dbg!(line_price).and_then(|price|
                                             acc.checked_add(&price.clone())
                                                .ok_or_else(|| Error::OpYieldedInvalidDecimalValue(
                                                                           Op::Add(acc, price))))))
//        Err(Error::PricingNotFoundAtQuantity(
//            prod,
//            NonZeroUsize::new(quant).expect(msg::ERR_INTERNAL_ZERO_USED_WITH_NON_ZERO_TYPE),
//            self.price_list.clone()))
}

    fn consolidate_product_list<I>(&self, product_list: I) -> Result<impl Iterator<Item = (Product, usize)>>
                                                              where I: IntoIterator,
                                                                    I::Item: Borrow<Product>, {
        product_list.into_iter()
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
                                  }.and_then(|_| Ok(prod_quants))
                              })
                    .and_then(|q_prods| Ok(q_prods.into_iter()))
    }

    pub fn scan<I>(&self, product_list: I) -> Result<Decimal> where I: IntoIterator,
                                                                    I::Item: Borrow<Product>, {
        self.consolidate_product_list(product_list)?
            .try_fold(Decimal::from(0),
                      |acc, (prod, quant)| self.calc_line_total(prod, quant)
                                               .and_then(|line_total|
                                                   acc.checked_add(&line_total)
                                                      .ok_or_else(|| Error::OpYieldedInvalidDecimalValue(
                                                          Op::Add(acc, line_total)))))
    }
}
