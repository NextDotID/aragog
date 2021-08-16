use crate::derives::record::collection_attribute::CollectionNameAttribute;
use crate::derives::record::hook::Hook;
use crate::derives::record::hooks_container::HooksContainer;
use crate::parse_attribute::ParseAttribute;
use crate::to_tokenstream::ToTokenStream;
use proc_macro::TokenStream;

mod collection_attribute;
mod hook;
mod hook_data;
mod hooks_container;
mod operation;

pub fn impl_record_macro(ast: &syn::DeriveInput) -> TokenStream {
    let target_name = &ast.ident;

    let mut hooks = Vec::new();
    let mut collection_names = Vec::new();
    for attr in ast.attrs.iter() {
        Hook::parse_attribute(attr, None, &mut hooks);
        if let Some(cn) = CollectionNameAttribute::parse_attribute(attr) {
            collection_names.push(cn)
        }
    }
    if collection_names.len() > 1 {
        emit_call_site_error!("Only one collection_name attribute is allowed");
    }
    let collection_name = match collection_names.first() {
        None => quote! { stringify!(#target_name) },
        Some(CollectionNameAttribute(lit)) => quote! { #lit },
    };
    let container = HooksContainer::from(hooks);
    let container_quote = container.token_stream();
    #[cfg(feature = "blocking")]
    let gen = quote! {
        impl Record for #target_name {
             const COLLECTION_NAME :&'static str = #collection_name;

            #container_quote
        }
    };
    #[cfg(not(feature = "blocking"))]
    let gen = quote! {
        #[aragog::async_trait::async_trait]
        impl Record for #target_name {
            const COLLECTION_NAME :&'static str = #collection_name;

            #container_quote
        }
    };
    // Debug purpose
    // println!("{}", gen);
    gen.into()
}
