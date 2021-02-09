mod command;
mod operation;

use crate::derives::validate::command::ValidateCommand;
use proc_macro::TokenStream;
use std::borrow::Borrow;

use syn::{Data, Fields};

pub fn impl_validate_macro(ast: &syn::DeriveInput) -> Result<TokenStream, String> {
    let target_name = &ast.ident;

    let mut commands = Vec::new();
    // We parse the struct attributes (#[validate(func("my_func"))])
    for attr in ast.attrs.iter() {
        ValidateCommand::parse_attribute(attr, None, &mut commands);
    }
    match ast.data.borrow() {
        Data::Struct(data) => match data.fields.borrow() {
            Fields::Named(named_fields) => {
                // We parse the field attributes
                for field in named_fields.named.iter() {
                    for attr in field.attrs.iter() {
                        ValidateCommand::parse_attribute(attr, Some(field), &mut commands);
                    }
                }
            }
            _ => {}
        },
        _ => return Err("Only Structs can derive `Validate`".to_string()),
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
    Ok(gen.into())
}
