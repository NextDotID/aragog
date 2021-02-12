use crate::derives::record::hook::Hook;
use crate::derives::record::hooks_container::HooksContainer;
use crate::parse_attribute::ParseAttribute;
use crate::to_tokenstream::ToTokenStream;
use proc_macro::TokenStream;
use std::borrow::Borrow;
use syn::Data;

mod hook;
mod hook_data;
mod hooks_container;

pub fn impl_record_macro(ast: &syn::DeriveInput) -> Result<TokenStream, String> {
    let target_name = &ast.ident;

    match ast.data.borrow() {
        Data::Struct(_elem) => {}
        _ => return Err("Only Structs can derive `Record`".to_string()),
    }

    let mut hooks = Vec::new();
    for attr in ast.attrs.iter() {
        Hook::parse_attribute(attr, None, &mut hooks);
    }
    let container = HooksContainer::from(hooks);
    let container_quote = container.token_stream();
    #[cfg(feature = "blocking")]
    let gen = quote! {
        impl Record for #target_name {
            fn collection_name() -> &'static str { stringify!(#target_name)  }

            #container_quote
        }
    };
    #[cfg(not(feature = "blocking"))]
    let gen = quote! {
        #[aragog::async_trait::async_trait]
        impl Record for #target_name {
            fn collection_name() -> &'static str { stringify!(#target_name)  }

            #container_quote
        }
    };
    // Debug purpose
    // println!("{}", gen);
    Ok(gen.into())
}
