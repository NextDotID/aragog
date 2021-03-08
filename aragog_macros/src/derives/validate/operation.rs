use crate::to_tokenstream::ToTokenStream;
use crate::toolbox::{expect_field_name, expect_int_lit, expect_str_lit, expect_usize_lit};
use proc_macro2::{Span, TokenStream};
use std::fmt::{self, Display, Formatter};
use syn::{Field, Ident, Lit};

#[derive(Debug, Clone)]
pub enum Operation {
    MinLength { value: usize, field: String },
    MaxLength { value: usize, field: String },
    Length { value: usize, field: String },
    Regex { value: String, field: String },
    GreaterThan { value: i32, field: String },
    LesserThan { value: i32, field: String },
    GreaterOrEqual { value: i32, field: String },
    LesserOrEqual { value: i32, field: String },
    Function { func: String, field: Option<String> },
}

impl Operation {
    pub fn parse(str: &str, lit: &Lit, field: Option<&Field>) -> Result<Self, String> {
        let res = match str {
            "min_length" => {
                let value = expect_usize_lit(lit)?;
                let field = expect_field_name(field)?;
                Self::MinLength { value, field }
            }
            "max_length" => {
                let value = expect_usize_lit(lit)?;
                let field = expect_field_name(field)?;
                Self::MaxLength { value, field }
            }
            "length" => {
                let value = expect_usize_lit(lit)?;
                let field = expect_field_name(field)?;
                Self::Length { value, field }
            }
            "regex" => {
                let value = expect_str_lit(lit)?;
                let field = expect_field_name(field)?;
                Self::Regex { value, field }
            }
            "greater_than" => {
                let value = expect_int_lit(lit)?;
                let field = expect_field_name(field)?;
                Self::GreaterThan { value, field }
            }
            "lesser_than" => {
                let value = expect_int_lit(lit)?;
                let field = expect_field_name(field)?;
                Self::LesserThan { value, field }
            }
            "greater_or_equal" => {
                let value = expect_int_lit(lit)?;
                let field = expect_field_name(field)?;
                Self::GreaterOrEqual { value, field }
            }
            "lesser_or_equal" => {
                let value = expect_int_lit(lit)?;
                let field = expect_field_name(field)?;
                Self::LesserOrEqual { value, field }
            }
            "func" => {
                let func = expect_str_lit(lit)?;
                let field = match field {
                    Some(f) => Some(f.ident.as_ref().unwrap().to_string()),
                    None => None,
                };
                Self::Function { func, field }
            }
            _ => return Err("Can't find a valid operation for field validation".to_string()),
        };
        Ok(res)
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operation::MinLength { .. } => "min_length",
                Operation::MaxLength { .. } => "max_length",
                Operation::Length { .. } => "length",
                Operation::Regex { .. } => "regex",
                Operation::GreaterThan { .. } => "greater_than",
                Operation::LesserThan { .. } => "lesser_than",
                Operation::GreaterOrEqual { .. } => "greater_or_equal",
                Operation::LesserOrEqual { .. } => "lesser_or_equal",
                Operation::Function { .. } => "func",
            }
        )
    }
}

impl ToTokenStream for Operation {
    fn token_stream(self) -> TokenStream {
        let stream = match self {
            Self::MinLength { value, field } => {
                let field_ident = Ident::new(&field, Span::call_site());
                quote! {
                    Self::validate_min_len(#field, &self.#field_ident, #value, errors);
                }
            }
            Self::MaxLength { value, field } => {
                let field_ident = Ident::new(&field, Span::call_site());
                quote! {
                    Self::validate_max_len(#field, &self.#field_ident, #value, errors);
                }
            }
            Self::Length { value, field } => {
                let field_ident = Ident::new(&field, Span::call_site());
                quote! {
                    Self::validate_len(#field, &self.#field_ident, #value, errors);
                }
            }
            Self::Regex { value, field } => {
                let field_ident = Ident::new(&field, Span::call_site());
                quote! {
                    Self::validate_regex(#field, &self.#field_ident, #value, errors);
                }
            }
            Self::GreaterThan { value, field } => {
                let field_ident = Ident::new(&field, Span::call_site());
                quote! {
                    Self::validate_greater_than(#field, self.#field_ident as i32, #value, errors);
                }
            }
            Self::LesserThan { value, field } => {
                let field_ident = Ident::new(&field, Span::call_site());
                quote! {
                    Self::validate_lesser_than(#field, self.#field_ident as i32, #value, errors);
                }
            }
            Self::GreaterOrEqual { value, field } => {
                let field_ident = Ident::new(&field, Span::call_site());
                quote! {
                    Self::validate_greater_or_equal_to(#field, self.#field_ident as i32, #value, errors);
                }
            }
            Self::LesserOrEqual { value, field } => {
                let field_ident = Ident::new(&field, Span::call_site());
                quote! {
                    Self::validate_lesser_or_equal_to(#field, self.#field_ident as i32, #value, errors);
                }
            }
            Self::Function { func, field } => {
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
        };
        stream
    }
}
