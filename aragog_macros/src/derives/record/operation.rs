use crate::parse_operation::{OperationValue, ParseOperation};
use crate::toolbox::{expect_bool_lit, expect_str_lit, get_ident};
use syn::{spanned::Spanned, Path};

const FORBIDDEN_FUNCTIONS: [&str; 6] = [
    "before_create_hook",
    "before_save_hook",
    "before_delete_hook",
    "after_create_hook",
    "after_save_hook",
    "after_delete_hook",
];

#[derive(Debug, Clone)]
pub enum HookOperation {
    Func(String),
    IsAsync(bool),
    DbAccess(bool),
}

impl ParseOperation for HookOperation {
    fn parse(path: &Path, value: Option<OperationValue>, field: Option<String>) -> Option<Self> {
        Self::expect_no_field(path, field)?;
        let ident = get_ident(path)?;
        let lit = Self::expect_literal_value(path, value)?;
        let res = match ident.as_str() {
            "func" => {
                let func = expect_str_lit(&lit)?;
                if FORBIDDEN_FUNCTIONS.contains(&func.as_str()) {
                    emit_error!(
                        lit.span(),
                        "Please use a different method name than `{}` \
                        as it may lead to unexpected behavior",
                        func
                    );
                    return None;
                }
                Self::Func(func)
            }
            "db_access" => {
                let value = expect_bool_lit(&lit)?;
                Self::DbAccess(value)
            }
            "is_async" => {
                let value = expect_bool_lit(&lit)?;
                Self::IsAsync(value)
            }
            _ => {
                emit_error!(path.span(), "Can't find a valid operation");
                return None;
            }
        };
        Some(res)
    }
}
