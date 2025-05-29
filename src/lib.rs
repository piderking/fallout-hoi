pub mod lexer;

pub mod test {
    use proc_macro2::TokenStream;
    use std::str::FromStr;
    use syn::ItemFn;
    #[test]
    pub fn test_macro() -> () {
        let source_code = "fn foo() {}";
        let token_stream = TokenStream::from_str(source_code).ok().unwrap();
        let parsed_function: ItemFn = syn::parse2(token_stream).ok().unwrap();
    }
}
