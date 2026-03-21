use proc_macro::TokenStream;

#[proc_macro_derive(ErrorUnion)]
pub fn derive_error_union(_input: TokenStream) -> TokenStream {
    // TODO: implement with syn + quote.
    // v1 behavior should support only enums whose variants are tuple variants
    // with exactly one field of type error_union::Located<T>.
    TokenStream::new()
}
