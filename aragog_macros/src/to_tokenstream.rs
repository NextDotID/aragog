use proc_macro2::TokenStream;

pub trait ToTokenStream {
    fn token_stream(self) -> TokenStream;
}
