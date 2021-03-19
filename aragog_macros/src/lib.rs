extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate proc_macro_error;

use proc_macro::TokenStream;
use std::borrow::Borrow;

use syn::{self, Data, DeriveInput, Fields};

use crate::derives::{impl_edge_record_macro, impl_record_macro, impl_validate_macro};

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

#[proc_macro_error]
#[proc_macro_derive(EdgeRecord)]
pub fn edge_record_macro_derive(attr: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(attr).unwrap();

    let mut has_from = false;
    let mut has_to = false;
    match ast.data.borrow() {
        Data::Struct(elem) => match elem.fields.borrow() {
            Fields::Named(named_fields) => {
                for named_field in named_fields.named.iter() {
                    match named_field.ident.borrow() {
                        Some(ident) => {
                            let field_name = &ident.to_string();
                            if field_name == "_to" {
                                has_to = true
                            } else if field_name == "_from" {
                                has_from = true
                            }
                        }
                        None => (),
                    }
                }
            }
            _ => {}
        },
        _ => emit_call_site_error!("Only Structs can derive `EdgeRecord`"),
    }
    if !has_from || !has_to {
        emit_call_site_error!("`EdgeRecord` derived structs require a `_from` and `_to` fields")
    }
    // Add from/to methods
    impl_edge_record_macro(&ast)
}
