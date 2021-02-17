use crate::derives::record::hook_data::{HookData, HookDataBuilder};
use crate::parse_attribute::ParseAttribute;
use crate::symbol::{Symbol, HOOK_ATTR_SYMBOL};
use crate::toolbox::get_ident;
use syn::{spanned::Spanned, Field, Meta, NestedMeta};

#[derive(Clone, Debug)]
pub enum Hook {
    BeforeAll(HookData),
    BeforeWrite(HookData),
    BeforeSave(HookData),
    BeforeCreate(HookData),
    BeforeDelete(HookData),
    AfterAll(HookData),
    AfterWrite(HookData),
    AfterSave(HookData),
    AfterCreate(HookData),
    AfterDelete(HookData),
}

impl ParseAttribute for Hook {
    fn symbol() -> Symbol {
        HOOK_ATTR_SYMBOL
    }

    fn parse_meta(meta: &Meta, _field: Option<&Field>) -> Result<Self, String> {
        match meta {
            Meta::List(list) => {
                // Defines our enum variant
                let ident = get_ident(&list.path)?;
                // By default everything is true
                let mut data = HookDataBuilder::default();
                // We now go through expected arguments (func, self_mutability, db_access)
                for nest in list.nested.iter() {
                    match nest {
                        NestedMeta::Meta(meta) => match data.parse_meta(meta) {
                            Ok(_) => (),
                            Err(error) => emit_error!(meta.span(), error),
                        },
                        _ => {
                            emit_error!(nest.span(), "Expected Meta");
                            return Err(String::from("Wrong format"));
                        }
                    }
                }
                let data = data.into_data()?;
                // We check the hook operation
                let res = match ident.as_str() {
                    "before_all" => Self::BeforeAll(data),
                    "before_write" => Self::BeforeWrite(data),
                    "before_save" => Self::BeforeSave(data),
                    "before_create" => Self::BeforeCreate(data),
                    "before_delete" => Self::BeforeDelete(data),
                    "after_save" => Self::AfterSave(data),
                    "after_create" => Self::AfterCreate(data),
                    "after_delete" => Self::AfterDelete(data),
                    "after_all" => Self::AfterAll(data),
                    "after_write" => Self::AfterWrite(data),
                    _ => {
                        return Err(String::from(
                            "Wrong identifier, specify a correct `before` or `after` hook.\
                        (`before_create` for example)",
                        ))
                    }
                };
                Ok(res)
            }
            _ => Err(String::from("Wrong format, expected parenthesis")),
        }
    }
}
