use proc_macro::TokenStream;

pub fn impl_record_macro(ast: &syn::DeriveInput) -> TokenStream {
    let target_name = &ast.ident;
    let gen = quote! {
        impl Record for #target_name {
            fn collection_name() -> &'static str { stringify!(#target_name)  }
        }
    };
    gen.into()
}
