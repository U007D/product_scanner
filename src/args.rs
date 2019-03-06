use structopt::StructOpt;

#[cfg(test)]
mod unit_tests;

/// Args is a structure representing the user's supplied command-line arguments supplied to the program as an
/// initialized data structure.  Types within the data structure are defined to be as restrictive as possible to
/// ensure only valid inputs are accepted.  Where types accurately reflect the application's constraints, conditional
/// logic, validation code & tests can all be safely eliminated, ∵ the invalid states of concern are not representable.
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Args {
    /// Sample argument (can be of any type).  Use `Option<some_type>` for optional arguments
    pub product_codes: usize,
}
