extern crate proc_macro;
#[macro_use]
extern crate proc_macro_error;

use proc_macro::TokenStream;
use std::borrow::Borrow;

use syn::{self, Data, DeriveInput, Fields};
use crate::derives::{impl_record_macro, impl_validate_macro, impl_edge_record_macro};

mod derives;

#[proc_macro_error]
#[proc_macro_derive(Record)]
pub fn record_macro_derive(attr: TokenStream) -> TokenStream {
    // Span for error
    let span = attr.clone().into_iter().next().unwrap().span();

    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(attr).unwrap();

    match ast.data.borrow() {
        Data::Struct(_elem) => {}
        _ => emit_error!(span, "Only Structs can derive `Record`")
    }
    // Build the trait implementation
    impl_record_macro(&ast)
}

#[proc_macro_derive(Validate)]
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
    // Span for error
    let span = attr.clone().into_iter().next().unwrap().span();
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(attr).unwrap();

    let mut has_from = false;
    let mut has_to = false;
    match ast.data.borrow() {
        Data::Struct(elem) => {
            match elem.fields.borrow() {
                Fields::Named(named_fields) => {
                    for named_field in named_fields.named.iter() {
                        match named_field.ident.borrow() {
                            Some(ident) => {
                                let field_name = &ident.to_string();
                                if field_name == "_to" { has_to = true } else if field_name == "_from" { has_from = true }
                            }
                            None => ()
                        }
                    }
                }
                _ => {}
            }
        }
        _ => emit_error!(span, "Only Structs can derive `EdgeRecord`")
    }
    if !has_from || !has_to {
        emit_error!(span, "`EdgeRecord` derived structs require a `_from` and `_to` fields")
    }
    // Add from/to methods
    impl_edge_record_macro(&ast)
}
