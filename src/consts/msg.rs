//! The [msg] module contains all user-facing message strings in the app, facilitating localization. The [msg] module is
//! further subdivided by locale, and re-exports a selected locale via `pub use <locale_module>::*;`.
pub use en_us::*;

mod en_us;
