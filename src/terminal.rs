use crate::{
    Decimal,
    Error,
    price_list::PriceList,
    Product,
    Result,
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

    pub fn scan<I>(&self, products: I) -> Result<f64> where I: IntoIterator<Item = Product> {
        products.into_iter()
                .try_fold(Decimal::from(0),
                          |acc, prod| self.price_list
                                          .find_product_pricing(&prod)
                                          .ok_or_else(|| Error::ProductNotFound(prod, self.price_list.clone()))
                                          .and_then(|price| acc + price)
    }
}
