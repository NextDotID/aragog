use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{spanned::Spanned, Lit, Meta, NestedMeta, Path};

#[derive(Clone)]
pub enum OperationValue {
    Lit(Lit),
    Path(Path),
}

impl OperationValue {
    pub fn parse(nest: &NestedMeta) -> Option<Self> {
        match nest {
            NestedMeta::Meta(meta) => match meta {
                Meta::List(list) => Some(Self::Path(list.path.clone())),
                Meta::NameValue(_) => {
                    emit_error!(
                        nest.span(),
                        "Wrong value, expected literal value or custom type got name value"
                    );
                    None
                }
                Meta::Path(p) => Some(Self::Path(p.clone())),
            },
            NestedMeta::Lit(lit) => Some(Self::Lit(lit.clone())),
        }
    }
}

impl ToTokens for OperationValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            OperationValue::Lit(lit) => lit.to_tokens(tokens),
            OperationValue::Path(path) => path.to_tokens(tokens),
        }
    }
}

pub trait ParseOperation: Sized {
    fn parse(path: &Path, lit: Option<OperationValue>, field: Option<String>) -> Option<Self>;

    fn expect_no_field(path: &Path, field: Option<String>) -> Option<()> {
        if field.is_none() {
            Some(())
        } else {
            emit_error!(path.span(), "This operation can't be placed on a field");
            None
        }
    }

    fn expect_field(path: &Path, field: Option<String>) -> Option<String> {
        if field.is_none() {
            emit_error!(path.span(), "This operation must be placed on a field");
        }
        field
    }

    fn expect_literal_value(path: &Path, value: Option<OperationValue>) -> Option<Lit> {
        match Self::expect_value(path, value)? {
            OperationValue::Lit(lit) => Some(lit),
            OperationValue::Path(_) => {
                emit_error!(path.span(), "Operation requires literal value, got a path");
                None
            }
        }
    }

    fn expect_value(path: &Path, value: Option<OperationValue>) -> Option<OperationValue> {
        if value.is_none() {
            emit_error!(path.span(), "Operation requires value");
        }
        value
    }

    fn expect_no_value(value: Option<OperationValue>) -> Option<()> {
        value.map_or(Some(()), |v| {
            emit_error!(v.span(), "Unexpected value");
            None
        })
    }
}
