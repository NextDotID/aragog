use proc_macro2::TokenStream;
use syn::{Field, Meta, NestedMeta};

use crate::derives::validate::operation::Operation;
use crate::parse_attribute::ParseAttribute;
use crate::symbol::{Symbol, VALIDATE_ATTR_SYMBOL};
use crate::to_tokenstream::ToTokenStream;
use crate::toolbox::get_ident;

#[derive(Debug, Clone)]
pub struct ValidateCommand {
    pub operation: Operation,
}

impl ParseAttribute for ValidateCommand {
    fn symbol() -> Symbol {
        VALIDATE_ATTR_SYMBOL
    }

    fn parse_meta(meta: &Meta, field: Option<&Field>) -> Result<Self, String> {
        let operation = match meta {
            Meta::NameValue(value) => {
                let ident = get_ident(&value.path)?;
                Operation::parse(&ident, &value.lit, field)?
            }
            Meta::List(list) => {
                let ident = get_ident(&list.path)?;
                let lit = match list.nested.first().unwrap() {
                    NestedMeta::Meta(_) => {
                        return Err(String::from("Wrong format, expected Lit value"))
                    }
                    NestedMeta::Lit(lit) => lit,
                };
                Operation::parse(&ident, &lit, field)?
            }
            _ => return Err(String::from("Wrong format: Add an argument")),
        };
        Ok(Self { operation })
    }
}

impl ToTokenStream for ValidateCommand {
    fn token_stream(self) -> TokenStream {
        self.operation.token_stream()
    }
}
