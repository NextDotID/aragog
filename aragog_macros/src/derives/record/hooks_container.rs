use crate::derives::record::hook::{Hook, HookType};
use crate::derives::record::hook_data::HookData;
use crate::to_tokenstream::ToTokenStream;
use proc_macro2::TokenStream;

#[derive(Clone, Default)]
pub struct HooksContainer {
    pub before_create: Vec<HookData>,
    pub before_save: Vec<HookData>,
    pub before_delete: Vec<HookData>,
    pub after_create: Vec<HookData>,
    pub after_save: Vec<HookData>,
    pub after_delete: Vec<HookData>,
}

impl From<Vec<Hook>> for HooksContainer {
    fn from(vec: Vec<Hook>) -> Self {
        let mut res = Self::default();
        for hook in vec {
            let data = hook.hook_data;
            match hook.hook_type {
                HookType::BeforeAll => {
                    res.before_create.push(data.clone());
                    res.before_save.push(data.clone());
                    res.before_delete.push(data.clone());
                }
                HookType::BeforeWrite => {
                    res.before_create.push(data.clone());
                    res.before_save.push(data.clone());
                }
                HookType::AfterAll => {
                    res.after_create.push(data.clone());
                    res.after_save.push(data.clone());
                    res.after_delete.push(data.clone());
                }
                HookType::AfterWrite => {
                    res.after_create.push(data.clone());
                    res.after_save.push(data.clone());
                }
                HookType::BeforeSave => res.before_save.push(data),
                HookType::BeforeCreate => res.before_create.push(data),
                HookType::BeforeDelete => res.before_delete.push(data),
                HookType::AfterSave => res.after_save.push(data),
                HookType::AfterCreate => res.after_create.push(data),
                HookType::AfterDelete => res.after_delete.push(data),
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
        quote
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
            fn before_create_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::Error>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #before_create_quote
                Ok(())
            }

            fn before_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::Error>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #before_save_quote
                Ok(())
            }

            fn before_delete_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::Error>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #before_delete_quote
                Ok(())
            }

            fn after_create_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::Error>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #after_create_quote
                Ok(())
            }

            fn after_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::Error>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #after_save_quote
                Ok(())
            }

            fn after_delete_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::Error>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #after_delete_quote
                Ok(())
            }
        };
        #[cfg(not(feature = "blocking"))]
        let gen = quote! {
            async fn before_create_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::Error>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #before_create_quote
                Ok(())
            }

            async fn before_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::Error>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #before_save_quote
                Ok(())
            }

            async fn before_delete_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::Error>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #before_delete_quote
                Ok(())
            }

            async fn after_create_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::Error>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #after_create_quote
                Ok(())
            }

            async fn after_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::Error>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #after_save_quote
                Ok(())
            }

            async fn after_delete_hook<D>(&mut self, db_accessor: &D) -> Result<(), aragog::Error>
            where
                D: aragog::DatabaseAccess + ?Sized {
                #after_delete_quote
                Ok(())
            }
        };
        gen
    }
}
