use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub fn xyz(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
