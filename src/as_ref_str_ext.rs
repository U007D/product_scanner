#[cfg(test)]
mod unit_tests;

use crate::{
    Error,
    Product,
    Result,
};

pub trait AsRefStrExt {
    fn as_product_list(&self) -> Result<Vec<Product>>;
}

impl<S: AsRef<str>> AsRefStrExt for S {
    fn as_product_list(&self) -> Result<Vec<Product>> {
       self.as_ref()
           .chars()
           .map(|c| match c.to_ascii_uppercase() {
               'A' => Ok(Product::A),
               'B' => Ok(Product::B),
               'C' => Ok(Product::C),
               'D' => Ok(Product::D),
               c => Err(Error::InvalidProductMnemonic(c)),
           })
           .collect()
    }
}
