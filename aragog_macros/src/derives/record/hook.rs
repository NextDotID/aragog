use syn::{Field, Path};

use crate::derives::record::hook_data::HookData;
use crate::derives::record::operation::HookOperation;
use crate::parse_attribute::ParseAttribute;
use crate::toolbox::expect_no_field_name;
use proc_macro2::Span;
use syn::spanned::Spanned;

#[derive(Clone, Debug)]
pub enum HookType {
    BeforeAll,
    BeforeWrite,
    BeforeSave,
    BeforeCreate,
    BeforeDelete,
    AfterAll,
    AfterWrite,
    AfterSave,
    AfterCreate,
    AfterDelete,
}

#[derive(Clone, Debug)]
pub struct Hook {
    pub hook_type: HookType,
    pub hook_data: HookData,
}

impl ParseAttribute for Hook {
    type AttributeOperation = HookOperation;

    fn init(path: &Path, field: Option<&Field>) -> Option<Self> {
        let ident = path.get_ident()?;
        expect_no_field_name(path.span(), field)?;
        let hook_type = match ident.to_string().as_str() {
            "before_all" => HookType::BeforeAll,
            "before_write" => HookType::BeforeWrite,
            "before_save" => HookType::BeforeSave,
            "before_create" => HookType::BeforeCreate,
            "before_delete" => HookType::BeforeDelete,
            "after_all" => HookType::AfterAll,
            "after_write" => HookType::AfterWrite,
            "after_save" => HookType::AfterSave,
            "after_create" => HookType::AfterCreate,
            "after_delete" => HookType::AfterDelete,
            _ => return None,
        };
        Some(Self {
            hook_type,
            hook_data: HookData {
                func: None,
                database_access: None,
                is_async: None,
            },
        })
    }

    fn field(&self) -> Option<String> {
        None
    }

    fn add_operation(&mut self, span: Span, operation: Self::AttributeOperation) {
        match operation {
            HookOperation::Func(func) => self.hook_data.edit_func(span, &func),
            HookOperation::IsAsync(v) => self.hook_data.edit_is_async(span, v),
            HookOperation::DbAccess(v) => self.hook_data.edit_db_access(span, v),
        }
    }

    fn validate(&self, span: Span) -> bool {
        if self.hook_data.func.is_none() {
            emit_error!(span, "Missing function for {:?} hook", self.hook_type);
            false
        } else {
            true
        }
    }
}
