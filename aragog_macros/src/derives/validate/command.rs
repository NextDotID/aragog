use proc_macro2::{Span, TokenStream};
use syn::Ident;
use syn::{spanned::Spanned, Attribute, Field, Meta, NestedMeta, Path};

use crate::derives::validate::operation::Operation;
use crate::symbol::VALIDATE_ATTR_SYMBOL;

#[derive(Debug)]
pub struct ValidateCommand {
    pub operation: Operation,
}

impl ValidateCommand {
    pub fn token_stream(self) -> TokenStream {
        match self.operation {
            Operation::MinLength { value, field } => {
                let field_ident = Ident::new(&field, Span::call_site());
                quote! {
                    Self::validate_min_len(#field, &self.#field_ident, #value, errors);
                }
            }
            Operation::MaxLength { value, field } => {
                let field_ident = Ident::new(&field, Span::call_site());
                quote! {
                    Self::validate_max_len(#field, &self.#field_ident, #value, errors);
                }
            }
            Operation::Length { value, field } => {
                let field_ident = Ident::new(&field, Span::call_site());
                quote! {
                    Self::validate_len(#field, &self.#field_ident, #value, errors);
                }
            }
            Operation::Regex { value, field } => {
                let field_ident = Ident::new(&field, Span::call_site());
                quote! {
                    Self::validate_regex(#field, &self.#field_ident, #value, errors);
                }
            }
            Operation::GreaterThan { value, field } => {
                let field_ident = Ident::new(&field, Span::call_site());
                quote! {
                    Self::validate_greater_than(#field, self.#field_ident as i32, #value, errors);
                }
            }
            Operation::LesserThan { value, field } => {
                let field_ident = Ident::new(&field, Span::call_site());
                quote! {
                    Self::validate_lower_than(#field, self.#field_ident as i32, #value, errors);
                }
            }
            Operation::GreaterOrEqual { value, field } => {
                let field_ident = Ident::new(&field, Span::call_site());
                quote! {
                    Self::validate_greater_or_equal_to(#field, self.#field_ident as i32, #value, errors);
                }
            }
            Operation::LesserOrEqual { value, field } => {
                let field_ident = Ident::new(&field, Span::call_site());
                quote! {
                    Self::validate_lesser_or_equal_to(#field, self.#field_ident as i32, #value, errors);
                }
            }
            Operation::Function { func, field } => {
                let func_ident = Ident::new(&func, Span::call_site());
                match field {
                    Some(field) => {
                        let field_ident = Ident::new(&field, Span::call_site());
                        quote! {
                            Self::#func_ident(&#field, &self.#field_ident, errors);
                        }
                    }
                    None => quote! {
                        self.#func_ident(errors);
                    },
                }
            }
        }
    }

    fn get_ident(path: &Path) -> Result<String, String> {
        let segment_len = path.segments.len();
        let segment = if segment_len == 1 {
            path.segments.first().unwrap()
        } else {
            if segment_len == 0 {
                return Err(String::from("Please add a segment"));
            } else {
                return Err(String::from("Too many segments"));
            }
        };
        Ok(segment.ident.to_string())
    }

    pub fn parse(meta: &Meta, field_name: Option<String>) -> Result<Self, String> {
        let operation = match meta {
            Meta::NameValue(value) => {
                let ident = Self::get_ident(&value.path)?;
                Operation::parse(&ident, &value.lit, field_name)?
            }
            Meta::List(list) => {
                let ident = Self::get_ident(&list.path)?;
                let lit = match list.nested.first().unwrap() {
                    NestedMeta::Meta(_) => {
                        return Err(String::from("Wrong format, expected Lit value"))
                    }
                    NestedMeta::Lit(lit) => lit,
                };
                Operation::parse(&ident, &lit, field_name)?
            }
            _ => return Err(String::from("Wrong format, expected Named value")),
        };
        Ok(Self { operation })
    }

    pub fn parse_attribute(attr: &Attribute, field: Option<&Field>, commands: &mut Vec<Self>) {
        if attr.path != VALIDATE_ATTR_SYMBOL {
            return;
        }
        match attr.parse_meta() {
            Ok(meta) => match meta {
                Meta::List(list) => {
                    for nest in list.nested.iter() {
                        match nest {
                            NestedMeta::Meta(meta) => {
                                let field_name = match field {
                                    None => None,
                                    Some(f) => Some(f.ident.as_ref().unwrap().to_string()),
                                };
                                let command = match ValidateCommand::parse(meta, field_name) {
                                    Ok(c) => c,
                                    Err(error) => {
                                        emit_error!(meta.span(), error);
                                        continue;
                                    }
                                };
                                commands.push(command);
                            }
                            NestedMeta::Lit(_) => {
                                emit_error!(nest.span(), "Expected meta, not lit")
                            }
                        }
                    }
                }
                _ => {
                    emit_error!(attr.span(), "Expected a meta list. Add valid operations")
                }
            },
            Err(error) => emit_error!(
                error.span(),
                format!("Failed to parse attribute: {}", error)
            ),
        }
    }
}
