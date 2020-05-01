mod macros;

use proc_macro::TokenStream;

#[proc_macro_derive(DisplayFromDebug)]
pub fn display_from_debug_derive(input: TokenStream) -> TokenStream {
    macros::display_from_debug_derive(input)
}
