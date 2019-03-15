/// `PriceScanner` product definition.  Defining products as a type provides a safer, more reliable interface.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Product {
    /// Fictional Product A
    A,
    /// Fictional Product B
    B,
    /// Fictional Product C
    C,
    /// Fictional Product D
    D,
}
