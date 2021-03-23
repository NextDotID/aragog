#![forbid(unsafe_code)]
#![deny(warnings)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate proc_macro_error;

use proc_macro::TokenStream;

use syn::{self, DeriveInput};

use crate::derives::{impl_record_macro, impl_validate_macro};

mod derives;
mod parse_attribute;
mod parse_operation;
mod to_tokenstream;
mod toolbox;

#[proc_macro_error]
#[proc_macro_derive(
    Record,
    attributes(
        before_create,
        before_save,
        before_write,
        before_delete,
        before_all,
        after_create,
        after_save,
        after_delete,
        after_write,
        after_all,
    )
)]
pub fn record_macro_derive(attr: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(attr).unwrap();

    // Build the trait implementation
    impl_record_macro(&ast)
}

#[proc_macro_error]
#[proc_macro_derive(Validate, attributes(validate, validate_each))]
pub fn validate_macro_derive(attr: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(attr).unwrap();

    // Build the trait implementation
    impl_validate_macro(&ast)
}
