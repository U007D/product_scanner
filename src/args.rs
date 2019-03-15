use structopt::StructOpt;

#[cfg(test)]
mod unit_tests;

/// Product scanner is a robust implementation of a exercise detailed in this crates notes (use `cargo doc --open` to
/// see them).  Currently the interface is exercised through unit tests (use `cargo test` to invoke them), but this
/// could be expanded to include the command line and/or API/RPC calls.  Please see the `README.md` document in the root
/// of this crate for more information.
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Args {
    /// A command-line interface to construct a price and purchase list is not currently implemented
    pub tbd: String,
}
