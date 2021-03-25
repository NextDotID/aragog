use crate::parse_operation::{OperationValue, ParseOperation};
use crate::toolbox::{expect_str_lit, expect_usize_lit, get_ident};
use proc_macro2::{Span, TokenStream};
use std::fmt::{self, Display, Formatter};
use syn::{spanned::Spanned, Ident, Path};

#[derive(Clone)]
pub enum Operation {
    MinLength {
        value: usize,
        field: String,
    },
    MaxLength {
        value: usize,
        field: String,
    },
    Length {
        value: usize,
        field: String,
    },
    MinCount {
        value: usize,
        field: String,
    },
    MaxCount {
        value: usize,
        field: String,
    },
    Count {
        value: usize,
        field: String,
    },
    Regex {
        value: String,
        field: String,
    },
    GreaterThan {
        value: OperationValue,
        field: String,
    },
    LesserThan {
        value: OperationValue,
        field: String,
    },
    GreaterOrEqual {
        value: OperationValue,
        field: String,
    },
    LesserOrEqual {
        value: OperationValue,
        field: String,
    },
    Function {
        func: String,
        field: Option<String>,
    },
    CallValidations {
        field: String,
    },
    IsSome {
        field: String,
    },
    IsNone {
        field: String,
    },
}

impl ParseOperation for Operation {
    fn parse(path: &Path, value: Option<OperationValue>, field: Option<String>) -> Option<Self> {
        let str = get_ident(path)?;
        let res = match str.as_str() {
            "min_length" => {
                let value = expect_usize_lit(&Self::expect_literal_value(path, value)?)?;
                let field = Self::expect_field(path, field)?;
                Self::MinLength { value, field }
            }
            "max_length" => {
                let value = expect_usize_lit(&Self::expect_literal_value(path, value)?)?;
                let field = Self::expect_field(path, field)?;
                Self::MaxLength { value, field }
            }
            "length" => {
                let value = expect_usize_lit(&Self::expect_literal_value(path, value)?)?;
                let field = Self::expect_field(path, field)?;
                Self::Length { value, field }
            }
            "min_count" => {
                let value = expect_usize_lit(&Self::expect_literal_value(path, value)?)?;
                let field = Self::expect_field(path, field)?;
                Self::MinCount { value, field }
            }
            "max_count" => {
                let value = expect_usize_lit(&Self::expect_literal_value(path, value)?)?;
                let field = Self::expect_field(path, field)?;
                Self::MaxCount { value, field }
            }
            "count" => {
                let value = expect_usize_lit(&Self::expect_literal_value(path, value)?)?;
                let field = Self::expect_field(path, field)?;
                Self::Count { value, field }
            }
            "regex" => {
                let value = expect_str_lit(&Self::expect_literal_value(path, value)?)?;
                let field = Self::expect_field(path, field)?;
                Self::Regex { value, field }
            }
            "greater_than" => {
                let value = Self::expect_value(path, value)?;
                let field = Self::expect_field(path, field)?;
                Self::GreaterThan { value, field }
            }
            "lesser_than" => {
                let value = Self::expect_value(path, value)?;
                let field = Self::expect_field(path, field)?;
                Self::LesserThan { value, field }
            }
            "greater_or_equal" => {
                let value = Self::expect_value(path, value)?;
                let field = Self::expect_field(path, field)?;
                Self::GreaterOrEqual { value, field }
            }
            "lesser_or_equal" => {
                let value = Self::expect_value(path, value)?;
                let field = Self::expect_field(path, field)?;
                Self::LesserOrEqual { value, field }
            }
            "call_validations" => {
                Self::expect_no_value(value)?;
                let field = Self::expect_field(path, field)?;
                Self::CallValidations { field }
            }
            "is_some" => {
                Self::expect_no_value(value)?;
                let field = Self::expect_field(path, field)?;
                Self::IsSome { field }
            }
            "is_none" => {
                Self::expect_no_value(value)?;
                let field = Self::expect_field(path, field)?;
                Self::IsNone { field }
            }
            "func" => {
                let lit = Self::expect_literal_value(path, value)?;
                let func = expect_str_lit(&lit)?;
                if func == "validations" {
                    emit_error!(
                        lit.span(),
                        "Please use a different method name than `validations` \
                        as it may lead to unexpected behavior"
                    );
                    return None;
                }
                Self::Function { func, field }
            }
            _ => {
                emit_error!(
                    path.span(),
                    "Can't find a valid operation for validation attribute"
                );
                return None;
            }
        };
        Some(res)
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
                Operation::MinCount { .. } => "min_count",
                Operation::MaxCount { .. } => "max_count",
                Operation::Count { .. } => "count",
                Operation::Regex { .. } => "regex",
                Operation::GreaterThan { .. } => "greater_than",
                Operation::LesserThan { .. } => "lesser_than",
                Operation::GreaterOrEqual { .. } => "greater_or_equal",
                Operation::LesserOrEqual { .. } => "lesser_or_equal",
                Operation::Function { .. } => "func",
                Operation::CallValidations { .. } => "call_validations",
                Operation::IsSome { .. } => "is_some",
                Operation::IsNone { .. } => "is_none",
            }
        )
    }
}

impl Operation {
    fn field_token(
        field: &String,
        custom_token_stream: Option<TokenStream>,
        no_ref: bool,
    ) -> TokenStream {
        match custom_token_stream {
            None => {
                let field_ident = Ident::new(&field, Span::call_site());
                let res = quote! { self.#field_ident };
                res
            }
            Some(token) => {
                if no_ref {
                    let q = quote! { *#token };
                    q.into()
                } else {
                    token
                }
            }
        }
    }

    pub(crate) fn token_stream(self, custom_token: Option<TokenStream>) -> TokenStream {
        let stream = match self {
            Self::MinLength { value, field } => {
                let field_token = Self::field_token(&field, custom_token, false);
                quote! {
                    Self::validate_min_len(#field, &#field_token, #value, errors);
                }
            }
            Self::MaxLength { value, field } => {
                let field_token = Self::field_token(&field, custom_token, false);
                quote! {
                    Self::validate_max_len(#field, &#field_token, #value, errors);
                }
            }
            Self::Length { value, field } => {
                let field_token = Self::field_token(&field, custom_token, false);
                quote! {
                    Self::validate_len(#field, &#field_token, #value, errors);
                }
            }
            Self::MinCount { value, field } => {
                let field_token = Self::field_token(&field, custom_token, false);
                quote! {
                    Self::validate_min_count(#field, #field_token.iter(), #value, errors);
                }
            }
            Self::MaxCount { value, field } => {
                let field_token = Self::field_token(&field, custom_token, false);
                quote! {
                    Self::validate_max_count(#field, #field_token.iter(), #value, errors);
                }
            }
            Self::Count { value, field } => {
                let field_token = Self::field_token(&field, custom_token, false);
                quote! {
                    Self::validate_count(#field, #field_token.iter(), #value, errors);
                }
            }
            Self::Regex { value, field } => {
                let field_token = Self::field_token(&field, custom_token, false);
                quote! {
                    Self::validate_regex(#field, &#field_token, #value, errors);
                }
            }
            Self::GreaterThan { value, field } => {
                let field_token = Self::field_token(&field, custom_token, true);
                quote! {
                    Self::validate_greater_than(#field, #field_token, #value, errors);
                }
            }
            Self::LesserThan { value, field } => {
                let field_token = Self::field_token(&field, custom_token, true);
                quote! {
                    Self::validate_lesser_than(#field, #field_token, #value, errors);
                }
            }
            Self::GreaterOrEqual { value, field } => {
                let field_token = Self::field_token(&field, custom_token, true);
                quote! {
                    Self::validate_greater_or_equal_to(#field, #field_token, #value, errors);
                }
            }
            Self::LesserOrEqual { value, field } => {
                let field_token = Self::field_token(&field, custom_token, true);
                quote! {
                    Self::validate_lesser_or_equal_to(#field, #field_token, #value, errors);
                }
            }
            Self::CallValidations { field } => {
                let field_token = Self::field_token(&field, custom_token, false);
                quote! {
                    #field_token.validations(errors);
                }
            }
            Self::IsSome { field } => {
                let field_token = Self::field_token(&field, custom_token, false);
                quote! {
                    Self::validate_field_presence(#field, &#field_token, errors);
                }
            }
            Self::IsNone { field } => {
                let field_token = Self::field_token(&field, custom_token, false);
                quote! {
                    Self::validate_field_absence(#field, &#field_token, errors);
                }
            }
            Self::Function { func, field } => {
                let func_ident = Ident::new(&func, Span::call_site());
                match field {
                    Some(field) => {
                        let field_token = Self::field_token(&field, custom_token, false);
                        quote! {
                            Self::#func_ident(#field, &#field_token, errors);
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
