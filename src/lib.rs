extern crate proc_macro;
use proc_macro::TokenStream;

/// This holds all the `extern "C"` declarations for the assembly code that the
/// crate can provide.
#[warn(missing_docs)]
pub mod fn_declarations;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
