use proc_macro2::{Span, TokenStream};
use syn::{Ident, Lit, Meta, NestedMeta};

use crate::to_tokenstream::ToTokenStream;
use crate::toolbox::{expect_bool_lit, expect_str_lit, get_ident};

#[derive(Clone, Debug)]
pub struct HookDataBuilder {
    pub func: Option<String>,
    pub database_access: Option<bool>,
    pub is_async: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct HookData {
    pub func: String,
    pub database_access: bool,
    pub is_async: bool,
}

impl HookDataBuilder {
    fn edit_func(&mut self, value: String) -> Result<(), String> {
        if self.func.is_some() {
            return Err(String::from("Can't have multiple `func` identifiers"));
        }
        self.func = Some(value);
        Ok(())
    }

    fn edit_database_access(&mut self, value: bool) -> Result<(), String> {
        if self.database_access.is_some() {
            return Err(String::from(
                "Can't have multiple `database_access` identifiers",
            ));
        }
        self.database_access = Some(value);
        Ok(())
    }

    fn edit_is_async(&mut self, value: bool) -> Result<(), String> {
        if self.is_async.is_some() {
            return Err(String::from("Can't have multiple `is_async` identifiers"));
        }
        self.is_async = Some(value);
        Ok(())
    }

    fn handle_ident_and_lit(&mut self, ident: &str, lit: &Lit) -> Result<(), String> {
        match ident {
            "func" => {
                let value = expect_str_lit(lit)?;
                self.edit_func(value)?;
            }
            "db_access" => {
                let value = expect_bool_lit(lit)?;
                self.edit_database_access(value)?;
            }
            "is_async" => {
                let value = expect_bool_lit(lit)?;
                self.edit_is_async(value)?;
            }
            _ => {
                return Err(format!(
                    "Wrong identifier `{}`: expected `func`, `is_async` or `db_access`",
                    ident
                ));
            }
        };
        Ok(())
    }

    pub fn parse_meta(&mut self, meta: &Meta) -> Result<(), String> {
        match meta {
            Meta::NameValue(value) => {
                let ident = get_ident(&value.path)?;
                self.handle_ident_and_lit(&ident, &value.lit)?;
            }
            Meta::List(list) => {
                let ident = get_ident(&list.path)?;
                let lit = match list.nested.first().unwrap() {
                    NestedMeta::Meta(_) => {
                        return Err(String::from("Wrong format, expected literal value"));
                    }
                    NestedMeta::Lit(lit) => lit,
                };
                self.handle_ident_and_lit(&ident, &lit)?;
            }
            _ => {
                return Err(String::from(
                    "Wrong format, expected Named value: Add an argument",
                ));
            }
        };
        Ok(())
    }

    pub fn into_data(self) -> Result<HookData, String> {
        match self.func {
            Some(func) => Ok(HookData {
                func,
                database_access: self.database_access.unwrap_or(false),
                #[cfg(feature = "blocking")]
                is_async: false,
                #[cfg(not(feature = "blocking"))]
                is_async: self.is_async.unwrap_or(false),
            }),
            None => Err(String::from("Missing `func` definition")),
        }
    }
}

impl Default for HookDataBuilder {
    fn default() -> Self {
        Self {
            func: None,
            database_access: None,
            is_async: None,
        }
    }
}

impl ToTokenStream for HookData {
    fn token_stream(self) -> TokenStream {
        let func_ident = Ident::new(&self.func, Span::call_site());
        let func = match self.database_access {
            true => quote! {
                self.#func_ident(db_accessor)
            },
            false => quote! {
                self.#func_ident()
            },
        };
        let res = match self.is_async {
            true => quote! {
                #func.await?;
            },
            false => quote! {
              #func?;
            },
        };
        res.into()
    }
}
