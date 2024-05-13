mod error_info;

use error_info::process_error_info;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ToErrorInfo, attributes(error_info))]
pub fn derive_to_error_info(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    process_error_info(input).into()
}
