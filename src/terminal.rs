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

    fn product_price_at_quantity(&self, prod_quant: (Product, usize)) -> Result<Decimal> {
        let (prod, quant) = prod_quant;
        self.price_list
            .find_product_pricing(prod)
            .ok_or_else(|| Error::ProductNotFound(prod, self.price_list.clone()))
            .and_then(|prices| prices.iter()
                .find(|price_map| price_map.quantity.get() <= quant)
                .ok_or_else(||
                    Error::PricingNotFoundAtQuantity(prod,
                                                     NonZeroUsize::new(quant).expect(msg::ERR_INTERNAL_INFALLIBLE_VALID_CONSTANT),
                                                     self.price_list.clone())))
            .and_then(|price_map|
                Ok(price_map.unit_price.clone() * Decimal::from(quant)))
    }

    #[allow(clippy::integer_arithmetic)]
    fn normalized_product_list<I>(&self, products: I) -> Result<impl Iterator<Item = (Product, usize)>>
                                                         where I: IntoIterator,
                                                         I::Item: Borrow<Product>, {
        let mut sentinel: Result<HashMap<Product, usize>>;
        products.into_iter()
                .try_fold(HashMap::<Product, usize>::new(),
                          |mut prod_quants, prod| {
                              prod_quants.get_mut(*prod.borrow())
                                         .and_then(|)
                                         .or_else(|| prod_quants.insert(*prod.borrow(), 1)
                              let entry = prod_quants.entry(*prod.borrow());
                              let () = entry;
                              entry
                                         .and_modify(|v| {
                                             let v2 = v.clone();
                                                         (*v).checked_add(1)
                                                         .or_else(|| {
                                                             sentinel = Err(Error::OpYieldedUnrepresentableIntegerValue(Op::Add(v2, 1)));
                                                             None
                                                         })
                                                         .and_then(|v_inc| {
                                                             *v = v_inc;
                                                             sentinel = Ok(prod_quants);
                                                             Some(v_inc)
                                                         });
                                         })
                                         .or_insert_with(|| 1);
                              sentinel
                          })
                .and_then(|q_prods| Ok(q_prods.into_iter()))
    }

    pub fn scan<I>(&self, products: I) -> Result<Decimal> where I: IntoIterator,
                                                                I::Item: Borrow<Product>, {
        self.normalized_product_list(products)?
            .try_fold(Decimal::from(0),
                      |acc, prod_quant| self.product_price_at_quantity(prod_quant)
                                            .and_then(|line_total| acc.checked_add(&line_total)
                                                                      .ok_or_else(
                                                                          || Error::OpYieldedUnrepresentableDecimalValue(
                                                                              Op::Add(acc, line_total)))))
//        products.into_iter()
//                .try_fold(
//                    Decimal::from(0),
//                    |acc, prod|
//                        // find product
//                        self.price_list
//                            .find_product_pricing(*prod.borrow())
//                            .ok_or_else(|| Error::ProductNotFound(*prod.borrow(), self.price_list.clone()))
//                            .and_then(|prices|
//                                prices.iter()
//                                      .find(|m|
//                                          m.quantity <= NonZeroUsize::new(1)
//                                                                   .expect(msg::ERR_INTERNAL_INFALLIBLE_VALID_CONSTANT))
//                                      .ok_or_else(||
//                                          Error::PricingNotFoundAtQuantity(
//                                              *prod.borrow(),
//                                              NonZeroUsize::new(1)
//                                                         .expect(msg::ERR_INTERNAL_INFALLIBLE_VALID_CONSTANT),
//                                              self.price_list.clone())))
//                            // tally pricing
//                            .and_then(|price| acc.checked_add(&price.price)
//                                                 .ok_or_else(||
//                                                     Error::OpYieldedUnrepresentableValue(Op::Add(acc,
//                                                                                                  price.price.clone())))
//                            ))
    }
}
