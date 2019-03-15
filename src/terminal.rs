use std::{
    borrow::Borrow,
    collections::HashMap,
    num::NonZeroUsize,
};

use fraction::{
    CheckedAdd,
    CheckedMul,
};

use crate::{
    Decimal,
    Error,
    msg,
    non_zero_usize_ext::NonZeroUsizeExt,
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

    fn calc_line_total(&self, prod: Product, nz_quant: NonZeroUsize) -> Result<Decimal> {
        let mut quant = dbg!(nz_quant.get());
        self.price_list
            .product_price_list(prod)
            .ok_or_else(|| Error::ProductNotFound(prod, self.price_list.clone()))
            .and_then(|prod_price_list| {
                  prod_price_list.iter()
                                 .filter_map(|quant_price| {
                                     let price_list_quant = quant_price.quantity.get();
                                     match price_list_quant <= quant {
                                         true => {
                                             Some(quant.checked_div(price_list_quant)
                                                       .ok_or_else(|| Error::IntegerOverflow(Op::Div(quant,
                                                                                                     price_list_quant)))
                                                       .and_then(|line_quant|
                                                           Decimal::from(line_quant)
                                                                   .checked_mul(&quant_price.price)
                                                                   .ok_or_else(|| Error::DecimalOverflow(
                                                                       Op::Mul(Decimal::from(line_quant),
                                                                               quant_price.price.clone())))
                                                                   .and_then(|line_tot|
                                                                       quant.checked_rem(dbg!(price_list_quant))
                                                                            .ok_or_else(|| Error::IntegerOverflow(
                                                                                Op::Rem(quant, price_list_quant)))
                                                                            .and_then(|quant_rem| {
                                                                                quant = quant_rem;
                                                                                Ok(line_tot)
                                                                            }))))
                                         },
                                         false => None,
                                     }
                                 })
                                 .fold(
                                     Option::<Result<Decimal>>::None,
                                     |tot, line_tot|
                                        Some(line_tot.and_then(|lt|
                                            tot.map_or_else(|| Ok(lt.clone()),
                                                            |t_result|
                                                                t_result.and_then(|t|
                                                                    t.checked_add(&lt)
                                                                     .ok_or_else(||
                                                                         Error::DecimalOverflow(
                                                                             Op::Add(t, lt.clone()))))
                                                                     .and_then(Ok)))))
                                 .unwrap_or_else(||
                                     Err(Error::PricingNotFoundAtQuantity(prod,
                                                                          nz_quant,
                                                                          self.price_list.clone())))
            })
}

    fn consolidate_product_list<I>(&self, prod_list: I) -> Result<impl Iterator<Item = (Product, NonZeroUsize)>>
                                                              where I: IntoIterator,
                                                                    I::Item: Borrow<Product>, {
        prod_list.into_iter()
                 .try_fold(HashMap::<Product, NonZeroUsize>::new(),
                           |mut prod_quants, prod| {
                               let key = prod.borrow();
                               match prod_quants.get_mut(key) {
                                   Some(val) => {
                                       val.checked_add(1)
                                          .ok_or_else(|| Error::IntegerOverflow(Op::Add(val.get(), 1)))
                                          .and_then(|v| {
                                              *val = v;
                                              Ok(())
                                          })
                                   },
                                   None => {
                                       prod_quants.insert(*key,
                                                          NonZeroUsize::new(1)
                                                               .expect(msg::ERR_INTERNAL_ZERO_USED_WITH_NON_ZERO_TYPE));
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
                                               .and_then(|line_tot|
                                                   acc.checked_add(&line_tot)
                                                      .ok_or_else(||
                                                          Error::DecimalOverflow(Op::Add(acc, line_tot)))))
    }
}
