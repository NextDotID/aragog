use proc_macro2::Span;
use syn::{spanned::Spanned, Field, Lit, Path};

pub fn get_ident(path: &Path) -> Option<String> {
    let res = path.get_ident();
    if res.is_none() {
        emit_error!(path.span(), "Expected identifier");
    }
    res.map(ToString::to_string)
}

pub fn expect_field_name(span: Span, field: Option<&Field>) -> Option<String> {
    if let Some(v) = field {
        Some(v.ident.as_ref().unwrap().to_string())
    } else {
        emit_error!(span, "This attribute must be placed on a field");
        None
    }
}

pub fn expect_no_field_name(span: Span, field: Option<&Field>) -> Option<()> {
    if field.is_some() {
        emit_error!(span, "This attribute must be placed on a field");
        None
    } else {
        Some(())
    }
}

pub fn expect_usize_lit(lit: &Lit) -> Option<usize> {
    if let Lit::Int(val) = lit {
        Some(val.base10_parse().unwrap())
    } else {
        emit_error!(lit.span(), "Expected an usize value");
        None
    }
}

pub fn expect_str_lit(lit: &Lit) -> Option<String> {
    if let Lit::Str(val) = lit {
        Some(val.value())
    } else {
        emit_error!(lit.span(), "Expected a string value");
        None
    }
}

pub fn expect_bool_lit(lit: &Lit) -> Option<bool> {
    if let Lit::Bool(val) = lit {
        Some(val.value)
    } else {
        emit_error!(lit.span(), "Expected a boolean value");
        None
    }
}
