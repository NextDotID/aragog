use syn::{spanned::Spanned, Attribute, Field, Meta, NestedMeta};

use crate::symbol::Symbol;

pub trait ParseAttribute: Sized {
    fn symbol() -> Symbol;

    fn parse_meta(meta: &Meta, field: Option<&Field>) -> Result<Self, String>;

    fn parse_attribute(attr: &Attribute, field: Option<&Field>, container: &mut Vec<Self>) {
        if attr.path != Self::symbol() {
            return;
        }
        match attr.parse_meta() {
            Ok(meta) => match meta {
                Meta::List(list) => {
                    for nest in list.nested.iter() {
                        match nest {
                            NestedMeta::Meta(meta) => {
                                let operation = match Self::parse_meta(meta, field) {
                                    Ok(c) => c,
                                    Err(error) => {
                                        emit_error!(meta.span(), error);
                                        continue;
                                    }
                                };
                                container.push(operation);
                            }
                            NestedMeta::Lit(_) => {
                                emit_error!(nest.span(), "Expected meta, not lit")
                            }
                        }
                    }
                }
                _ => {
                    emit_error!(meta.span(), "Expected a meta list. Add valid operations")
                }
            },
            Err(error) => emit_error!(
                error.span(),
                format!("Failed to parse attribute: {}", error)
            ),
        }
    }
}
