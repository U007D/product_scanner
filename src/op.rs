use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub enum Op<T: Debug> {
    Add(T, T),
    Sub(T, T),
    Mul(T, T),
    Div(T, T),
}
