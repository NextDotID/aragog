use syn::{Field, Lit, Path};

pub fn get_ident(path: &Path) -> Result<String, String> {
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

pub fn expect_field_name(field: Option<&Field>) -> Result<String, String> {
    match field {
        Some(v) => Ok(v.ident.as_ref().unwrap().to_string()),
        None => Err(String::from("This attribute must be placed on a field")),
    }
}

pub fn expect_int_lit(lit: &Lit) -> Result<i32, String> {
    match lit {
        Lit::Int(val) => Ok(val.base10_parse().unwrap()),
        _ => {
            let error = "Expected an integer value".to_string();
            emit_error!(lit.span(), error);
            Err(error)
        }
    }
}

pub fn expect_usize_lit(lit: &Lit) -> Result<usize, String> {
    match lit {
        Lit::Int(val) => Ok(val.base10_parse().unwrap()),
        _ => {
            let error = "Expected an usize value".to_string();
            emit_error!(lit.span(), error);
            Err(error)
        }
    }
}

pub fn expect_str_lit(lit: &Lit) -> Result<String, String> {
    match lit {
        Lit::Str(val) => Ok(val.value()),
        _ => {
            let error = "Expected a string value".to_string();
            emit_error!(lit.span(), error);
            Err(error)
        }
    }
}

pub fn expect_bool_lit(lit: &Lit) -> Result<bool, String> {
    match lit {
        Lit::Bool(val) => Ok(val.value),
        _ => {
            let error = "Expected a boolean value".to_string();
            emit_error!(lit.span(), error);
            Err(error)
        }
    }
}
