use crate::derives::record::hook::Hook;
use crate::derives::record::hook_data::HookData;
use crate::to_tokenstream::ToTokenStream;
use proc_macro2::TokenStream;

#[derive(Debug, Clone)]
pub struct HooksContainer {
    pub before_create: Vec<HookData>,
    pub before_save: Vec<HookData>,
    pub before_delete: Vec<HookData>,
    pub after_create: Vec<HookData>,
    pub after_save: Vec<HookData>,
    pub after_delete: Vec<HookData>,
}

impl Default for HooksContainer {
    fn default() -> Self {
        Self {
            before_create: vec![],
            before_save: vec![],
            before_delete: vec![],
            after_create: vec![],
            after_save: vec![],
            after_delete: vec![],
        }
    }
}

impl From<Vec<Hook>> for HooksContainer {
    fn from(vec: Vec<Hook>) -> Self {
        let mut res = Self::default();
        for item in vec {
            match item {
                Hook::BeforeAll(data) => {
                    res.before_create.push(data.clone());
                    res.before_save.push(data.clone());
                    res.before_delete.push(data.clone());
                }
                Hook::BeforeWrite(data) => {
                    res.before_create.push(data.clone());
                    res.before_save.push(data.clone());
                }
                Hook::AfterAll(data) => {
                    res.after_create.push(data.clone());
                    res.after_save.push(data.clone());
                    res.after_delete.push(data.clone());
                }
                Hook::AfterWrite(data) => {
                    res.after_create.push(data.clone());
                    res.after_save.push(data.clone());
                }
                Hook::BeforeSave(data) => res.before_save.push(data),
                Hook::BeforeCreate(data) => res.before_create.push(data),
                Hook::BeforeDelete(data) => res.before_delete.push(data),
                Hook::AfterSave(data) => res.after_save.push(data),
                Hook::AfterCreate(data) => res.after_create.push(data),
                Hook::AfterDelete(data) => res.after_delete.push(data),
            }
        }
        res
    }
}

impl ToTokenStream for Vec<HookData> {
    fn token_stream(self) -> TokenStream {
        let mut quote = quote! {};
        for item in self {
            let item_quote = item.token_stream();
            quote = quote! {
                #quote
                #item_quote
            }
        }
        quote.into()
    }
}

impl ToTokenStream for HooksContainer {
    fn token_stream(self) -> TokenStream {
        let before_create_quote = self.before_create.token_stream();
        let before_save_quote = self.before_save.token_stream();
        let before_delete_quote = self.before_delete.token_stream();
        let after_create_quote = self.after_create.token_stream();
        let after_save_quote = self.after_save.token_stream();
        let after_delete_quote = self.after_delete.token_stream();
        #[cfg(feature = "blocking")]
        let gen = quote! {
            fn before_create_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::ServiceError>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #before_create_quote
                Ok(())
            }

            fn before_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::ServiceError>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #before_save_quote
                Ok(())
            }

            fn before_delete_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::ServiceError>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #before_delete_quote
                Ok(())
            }

            fn after_create_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::ServiceError>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #after_create_quote
                Ok(())
            }

            fn after_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::ServiceError>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #after_save_quote
                Ok(())
            }

            fn after_delete_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::ServiceError>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #after_delete_quote
                Ok(())
            }
        };
        #[cfg(not(feature = "blocking"))]
        let gen = quote! {
            async fn before_create_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::ServiceError>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #before_create_quote
                Ok(())
            }

            async fn before_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::ServiceError>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #before_save_quote
                Ok(())
            }

            async fn before_delete_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::ServiceError>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #before_delete_quote
                Ok(())
            }

            async fn after_create_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::ServiceError>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #after_create_quote
                Ok(())
            }

            async fn after_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::ServiceError>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #after_save_quote
                Ok(())
            }

            async fn after_delete_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::ServiceError>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #after_delete_quote
                Ok(())
            }
        };
        gen.into()
    }
}
