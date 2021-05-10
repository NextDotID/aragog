mod command;
mod operation;

use crate::derives::validate::command::ValidateCommand;
use proc_macro::TokenStream;
use std::borrow::Borrow;

use crate::parse_attribute::ParseAttribute;
use crate::to_tokenstream::ToTokenStream;
use syn::{spanned::Spanned, Data, Fields};

pub fn impl_validate_macro(ast: &syn::DeriveInput) -> TokenStream {
    let target_name = &ast.ident;

    let mut commands = Vec::new();
    // We parse the struct attributes (#[validate(func("my_func"))])
    for attr in ast.attrs.iter() {
        ValidateCommand::parse_attribute(attr, None, &mut commands);
    }
    match ast.data.borrow() {
        Data::Struct(data) => {
            if let Fields::Named(named_fields) = data.fields.borrow() {
                // We parse the field attributes
                for field in named_fields.named.iter() {
                    for attr in field.attrs.iter() {
                        ValidateCommand::parse_attribute(attr, Some(field), &mut commands);
                    }
                }
            }
        }
        Data::Enum(data) => {
            for variant in data.variants.iter() {
                if !variant.attrs.is_empty() {
                    emit_error!(
                        variant.span(),
                        "validation attributes on enum variants are not supported"
                    );
                }
                for field in variant.fields.iter() {
                    for attr in field.attrs.iter() {
                        emit_error!(
                            attr.span(),
                            "validation attributes on enum variants are not supported"
                        );
                    }
                }
            }
        }
        _ => {}
    }

    let mut validation_quote = quote! {};
    for command in commands.into_iter() {
        let operation = command.token_stream();
        validation_quote = quote! {
            #validation_quote
            #operation
        };
    }
    let gen = quote! {
        impl Validate for #target_name {
            fn validations(&self, errors: &mut Vec<String>) {
                #validation_quote
             }
        }
    };
    // Debug purposes
    // println!("{}", gen);
    gen.into()
}
