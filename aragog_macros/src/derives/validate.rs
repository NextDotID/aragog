use proc_macro::TokenStream;

pub fn impl_validate_macro(ast: &syn::DeriveInput) -> TokenStream {
    let target_name = &ast.ident;
    let gen = quote! {
        impl Validate for #target_name {
            fn validations(&self, _errors: &mut Vec<String>) { }
        }
    };
    gen.into()
}
