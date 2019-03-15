use std::{
    borrow::Borrow,
    collections::HashMap,
    num::NonZeroUsize,
};

use crate::{
    Decimal,
    Error,
    msg,
    price_list::PriceList,
    Product,
    Result
};

#[cfg(test)]
mod unit_tests;

/// `Terminal` represents the type which performs the computations yielding a total price for all goods to be purchased.
pub struct Terminal {
    price_list: PriceList,
}

impl Terminal {
    /// Constructor.
    pub fn new(price_list: PriceList) -> Self {
        Self {
            price_list,
        }
    }

    fn calc_line_total(&self, prod: Product, nz_quant: NonZeroUsize) -> Result<Decimal> {
        self.price_list
            .product_price_list(prod)
            .ok_or_else(|| Error::ProductNotFound(prod, self.price_list.clone()))
            .and_then(|prod_price_list| {
                  prod_price_list.iter()
                                 .scan(nz_quant.get(), |quant, quant_price| {
                                     let price_list_quant = quant_price.quantity.get();
                                     match price_list_quant <= *quant {
                                         true => {
                                             let line_quant = *quant / price_list_quant;
                                             *quant %= price_list_quant;
                                             Some(Some(Ok(Decimal::from(line_quant) * quant_price.price.clone())))
                                         },
                                         false => Some(None),
                                     }
                                 })
                                 .filter_map(|el| el)
                                 .fold(
                                     Option::<Result<Decimal>>::None,
                                     |tot, line_tot|
                                        Some(line_tot.and_then(|lt|
                                            tot.map_or_else(|| Ok(lt.clone()),
                                                            |t_result| t_result.and_then(|t| Ok(t + lt.clone()))))))
                                 .unwrap_or_else(||
                                     Err(Error::PricingNotFoundAtQuantity(prod,
                                                                          nz_quant,
                                                                          self.price_list.clone())))
            })
}

    fn collate_product_list<I>(&self, prod_list: I) -> Result<impl Iterator<Item = (Product, NonZeroUsize)>>
                                                       where I: IntoIterator,
                                                             I::Item: Borrow<Product>, {
        prod_list.into_iter()
                 .try_fold(HashMap::<Product, NonZeroUsize>::new(),
                           |mut prod_quants, prod| {
                               let key = prod.borrow();
                               match prod_quants.get_mut(key) {
                                   Some(val) => {
                                       *val = NonZeroUsize::new(usize::from(*val) + 1)
                                                           .expect(msg::ERR_INTERNAL_ZERO_USED_WITH_NON_ZERO_TYPE);
                                       Ok(())
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

    /// Given a list of products, collates them so that any applicable quantity pricing can be applied, providing the
    /// lowest possible total bill to the customer.
    pub fn scan<I>(&self, product_list: I) -> Result<Decimal> where I: IntoIterator,
                                                                    I::Item: Borrow<Product>, {
        self.collate_product_list(product_list)?
            .try_fold(Decimal::from(0),
                      |acc, (prod, quant)| self.calc_line_total(prod, quant)
                                               .and_then(|line_tot| Ok(acc + line_tot)))
    }
}
