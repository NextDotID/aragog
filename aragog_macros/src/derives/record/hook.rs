use crate::derives::record::hook_data::{HookData, HookDataBuilder};
use crate::parse_attribute::ParseAttribute;
use crate::symbol::{Symbol, HOOK_ATTR_SYMBOL};
use crate::toolbox::get_ident;
use syn::{spanned::Spanned, Field, Meta, NestedMeta};

#[derive(Clone, Debug)]
pub enum Hook {
    BeforeAll(HookData),
    BeforeSave(HookData),
    BeforeCreate(HookData),
    AfterAll(HookData),
    AfterSave(HookData),
    AfterCreate(HookData),
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
                    "before_save" => Self::BeforeSave(data),
                    "before_create" => Self::BeforeCreate(data),
                    "after_save" => Self::AfterSave(data),
                    "after_create" => Self::AfterCreate(data),
                    "after_all" => Self::AfterAll(data),
                    _ => return Err(String::from(
                        "Wrong identifier, expected one of:\
                         `before_all`,`before_create`, `before_save`, `after_all`, `after_create` or `after_save`"
                    ))
                };
                Ok(res)
            }
            _ => Err(String::from("Wrong format, expected parenthesis")),
        }
    }
}
