use proc_macro::TokenStream;

pub fn impl_edge_record_macro(ast: &syn::DeriveInput) -> TokenStream {
    let target_name = &ast.ident;

    let gen = quote! {
        impl EdgeRecord for #target_name {
            fn _from(&self) -> String { self._from.clone() }

            fn _to(&self) -> String { self._to.clone() }
        }
    };
    gen.into()
}
