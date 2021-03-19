use proc_macro2::Span;
use syn::{spanned::Spanned, Field, Lit, Path};

pub fn get_ident(path: &Path) -> Option<String> {
    match path.get_ident() {
        None => {
            emit_error!(path.span(), "Expected identifier");
            None
        }
        Some(i) => Some(i.to_string()),
    }
}

pub fn expect_field_name(span: Span, field: Option<&Field>) -> Option<String> {
    match field {
        Some(v) => Some(v.ident.as_ref().unwrap().to_string()),
        None => {
            emit_error!(span, "This attribute must be placed on a field");
            None
        }
    }
}

pub fn expect_no_field_name(span: Span, field: Option<&Field>) -> Option<()> {
    match field {
        Some(_) => {
            emit_error!(span, "This attribute must be placed on a field");
            None
        }
        None => Some(()),
    }
}

pub fn expect_usize_lit(lit: &Lit) -> Option<usize> {
    match lit {
        Lit::Int(val) => Some(val.base10_parse().unwrap()),
        _ => {
            emit_error!(lit.span(), "Expected an usize value");
            None
        }
    }
}

pub fn expect_str_lit(lit: &Lit) -> Option<String> {
    match lit {
        Lit::Str(val) => Some(val.value()),
        _ => {
            emit_error!(lit.span(), "Expected a string value");
            None
        }
    }
}

pub fn expect_bool_lit(lit: &Lit) -> Option<bool> {
    match lit {
        Lit::Bool(val) => Some(val.value),
        _ => {
            emit_error!(lit.span(), "Expected a boolean value");
            None
        }
    }
}
