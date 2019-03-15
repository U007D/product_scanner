/// Error message indicating an `OsString` was not able to be converted into a UTF-8 `String`.
pub const ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8: &str = "Error: supplied command-line argument not convertible to UTF-8";

/// Error message indicating builder was asked to build a `PriceList` without any `PriceList` entries.
pub const ERR_EMPTY_PRODUCT_PRICING_TABLE: &str = "Error: Cannot build an empty product pricing table";

/// Error message indicating an implementation bug--after just checking that a key was not present and holding an
/// exclusive lock on the table the entire time, the key somehow became present before we were able to add it.
pub const ERR_INTERNAL_KVP_ALREADY_PRESENT: &str = "Internal error: Unexpected key/value pair already present in product table";

/// Error message indicating a value has overflowed its type.
pub const ERR_UNREPRESENTABLE_VALUE: &str = "Error: Calculation yield unrepresentable result (result is too small or too large?)";

/// Error message indicating a product is not present in the `PriceList`.
pub const ERR_PRODUCT_NOT_FOUND: &str = "Error: Product was not found in the price list (product, price list)";

/// Error message indicating a product at quantity x was not found in the `PriceList`.  The `PriceList` may contain a
/// pricing entry for that same product at quantity y, where y > x, but because not enough of the item is being
/// purchased, no pricing information could be found.
/// To be sure this condition does not occur, ensure there is quantity 1 pricing for every product (in addition to other
/// quantities, if desired).
pub const ERR_PRICING_NOT_FOUND_AT_QUANTITY: &str = "Error: No pricing information for `product` at `quantity` in `product price list`: ";

/// Pricing can never be for a 0 quantity.  Following best practices, this software encodes invariants such as this
/// using the type system whenever possible, which makes invalid states impossible to represent, rather than relying on
/// runtime tests to (hopefully) check for the illegal condition.
/// This error message indicates an implementation bug where an author has set a `NonZero*` type to 0.
pub const ERR_INTERNAL_ZERO_USED_WITH_NON_ZERO_TYPE: &str = "Internal error: Invalid constant value supplied--cannot use 0 with a `NonZero*` type";

/// `AsRefStrExt::as_product_list()` is a helper function to translate a string of characters into a slice of
/// `Products`.  If a product which does not exist is provided as part of the input string, this message will
/// communicate the issue.
pub const ERR_INVALID_PRODUCT_MNEMONIC: &str = "Error: Invalid product mnemonic supplied.  No corresponding `Product` found";
