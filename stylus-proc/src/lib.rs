// Copyright 2022-2023, Offchain Labs, Inc.
// For licensing, see https://github.com/OffchainLabs/stylus-sdk-rs/blob/stylus/licenses/COPYRIGHT.md

use proc_macro::TokenStream;

/// Generates a pretty error message.
/// Note that this macro is declared before all modules so that they can use it.
macro_rules! error {
    ($tokens:expr, $($msg:expr),+ $(,)?) => {{
        let error = syn::Error::new(syn::spanned::Spanned::span(&$tokens), format!($($msg),+));
        return error.to_compile_error().into();
    }};
}

mod calls;
mod methods;
mod storage;
mod types;

#[proc_macro_attribute]
pub fn solidity_storage(attr: TokenStream, input: TokenStream) -> TokenStream {
    storage::solidity_storage(attr, input)
}

#[proc_macro]
pub fn sol_storage(input: TokenStream) -> TokenStream {
    storage::sol_storage(input)
}

#[proc_macro]
pub fn sol_interface(input: TokenStream) -> TokenStream {
    calls::sol_interface(input)
}

#[proc_macro_derive(Erase)]
pub fn derive_erase(input: TokenStream) -> TokenStream {
    storage::derive_erase(input)
}

#[proc_macro_derive(Entrypoint)]
pub fn derive_entrypoint(input: TokenStream) -> TokenStream {
    methods::entrypoint::derive_entrypoint(input)
}

#[proc_macro_attribute]
pub fn external(attr: TokenStream, input: TokenStream) -> TokenStream {
    methods::external::external(attr, input)
}