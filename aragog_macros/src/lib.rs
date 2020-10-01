extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Record)]
pub fn record_macro_derive(attr: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(attr).unwrap();

    // Build the trait implementation
    impl_record_macro(&ast)
}

fn impl_record_macro(ast: &syn::DeriveInput) -> TokenStream {
    let target_name = &ast.ident;
    let gen = quote! {
        impl Record for #target_name {
            fn collection_name() -> &'static str { stringify!(#target_name)  }
        }
    };
    gen.into()
}

#[proc_macro_derive(Validate)]
pub fn validate_macro_derive(attr: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(attr).unwrap();

    // Build the trait implementation
    impl_validate_macro(&ast)
}

fn impl_validate_macro(ast: &syn::DeriveInput) -> TokenStream {
    let target_name = &ast.ident;
    let gen = quote! {
        impl Validate for #target_name {
            fn validations(&self, _errors: &mut Vec<String>) { }
        }
    };
    gen.into()
}