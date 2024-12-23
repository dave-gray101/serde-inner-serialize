use serde_inner_serialize_core::inner_serializable_core;
use proc_macro_error::proc_macro_error;
use proc_macro::TokenStream;

#[proc_macro_error]
#[proc_macro_derive(InnerSerializable)]
pub fn derive_inner_serializable(input: TokenStream) -> TokenStream {
    inner_serializable_core(input.into()).into()
}
