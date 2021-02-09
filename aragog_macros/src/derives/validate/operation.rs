use std::fmt::{self, Display, Formatter};
use syn::Lit;

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
    Function(String),
}

impl Operation {
    pub fn parse(str: &str, lit: &Lit, field_name: Option<String>) -> Result<Self, String> {
        let res = match str {
            "min_length" => {
                let value = Self::expect_usize_lit(lit)?;
                let field = Self::expect_field_name(field_name)?;
                Self::MinLength { value, field }
            }
            "max_length" => {
                let value = Self::expect_usize_lit(lit)?;
                let field = Self::expect_field_name(field_name)?;
                Self::MaxLength { value, field }
            }
            "length" => {
                let value = Self::expect_usize_lit(lit)?;
                let field = Self::expect_field_name(field_name)?;
                Self::Length { value, field }
            }
            "regex" => {
                let value = Self::expect_str_lit(lit)?;
                let field = Self::expect_field_name(field_name)?;
                Self::Regex { value, field }
            }
            "greater_than" => {
                let value = Self::expect_int_lit(lit)?;
                let field = Self::expect_field_name(field_name)?;
                Self::GreaterThan { value, field }
            }
            "lesser_than" => {
                let value = Self::expect_int_lit(lit)?;
                let field = Self::expect_field_name(field_name)?;
                Self::LesserThan { value, field }
            }
            "greater_or_equal" => {
                let value = Self::expect_int_lit(lit)?;
                let field = Self::expect_field_name(field_name)?;
                Self::GreaterOrEqual { value, field }
            }
            "lesser_or_equal" => {
                let value = Self::expect_int_lit(lit)?;
                let field = Self::expect_field_name(field_name)?;
                Self::LesserOrEqual { value, field }
            }
            "func" => {
                let v = Self::expect_str_lit(lit)?;
                Self::expect_no_field_name(field_name)?;
                Self::Function(v)
            }
            _ => return Err("Can't find a valid operation for field validation".to_string()),
        };
        Ok(res)
    }

    fn expect_field_name(field: Option<String>) -> Result<String, String> {
        match field {
            Some(v) => Ok(v),
            None => Err(String::from("This attribute must be placed on a field")),
        }
    }

    fn expect_no_field_name(field: Option<String>) -> Result<(), String> {
        match field {
            Some(_) => Err(String::from(
                "This attribute must be placed on the struct definition",
            )),
            None => Ok(()),
        }
    }

    fn expect_int_lit(lit: &Lit) -> Result<i32, String> {
        match lit {
            Lit::Int(val) => Ok(val.base10_parse().unwrap()),
            _ => {
                let error = "Expected an integer value".to_string();
                emit_error!(lit.span(), error);
                Err(error)
            }
        }
    }

    fn expect_usize_lit(lit: &Lit) -> Result<usize, String> {
        match lit {
            Lit::Int(val) => Ok(val.base10_parse().unwrap()),
            _ => {
                let error = "Expected an integer value".to_string();
                emit_error!(lit.span(), error);
                Err(error)
            }
        }
    }

    fn expect_str_lit(lit: &Lit) -> Result<String, String> {
        match lit {
            Lit::Str(val) => Ok(val.value()),
            _ => {
                let error = "Expected a string value".to_string();
                emit_error!(lit.span(), error);
                Err(error)
            }
        }
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
                Operation::Function(_) => "func",
            }
        )
    }
}
