use proc_macro2::{Span, TokenStream};
use syn::Ident;

use crate::to_tokenstream::ToTokenStream;

#[derive(Clone)]
pub struct HookData {
    pub func: Option<String>,
    pub database_access: Option<bool>,
    pub is_async: Option<bool>,
}

impl HookData {
    pub fn edit_func(&mut self, span: Span, func: &str) {
        if self.func.is_some() {
            emit_error!(span, "Can't have multiple `func` identifiers");
            return;
        }
        self.func = Some(func.to_string());
    }

    pub fn edit_db_access(&mut self, span: Span, value: bool) {
        if self.database_access.is_some() {
            emit_error!(span, "Can't have multiple `database_access` identifiers",);
        }
        self.database_access = Some(value);
    }

    pub fn edit_is_async(&mut self, span: Span, value: bool) {
        if self.is_async.is_some() {
            emit_error!(span, "Can't have multiple `is_async` identifiers");
        }
        self.is_async = Some(value);
    }
}

impl ToTokenStream for HookData {
    fn token_stream(self) -> TokenStream {
        let func = match self.func {
            Some(f) => f,
            None => {
                emit_call_site_error!("Missing function for hook");
                return TokenStream::new();
            }
        };
        #[cfg(feature = "blocking")]
        let is_async = false;
        #[cfg(not(feature = "blocking"))]
        let is_async = self.is_async.unwrap_or(false);
        let db_access = self.database_access.unwrap_or(false);
        let func_ident = Ident::new(&func, Span::call_site());
        let func = match db_access {
            true => quote! {
                self.#func_ident(db_accessor)
            },
            false => quote! {
                self.#func_ident()
            },
        };
        let res = match is_async {
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
