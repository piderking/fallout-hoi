use proc_macro2::{LexError, TokenStream};
use std::{path::Path, str::FromStr};

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

trait Process {
    type Output: std::fmt::Debug;

    fn next(self) -> Self::Output;
}

#[derive(Debug)]
pub enum State {
    Entry(Entry),
    Built(Built),
    Display(String),
    Json(Box<Path>),
}

impl Process for State {
    type Output = Self;

    fn next(self) -> Self::Output {
        match self {
            State::Entry(entry) => State::Built(entry.next()),
            State::Built(built) => todo!(),
            State::Display(s) => todo!(),
            State::Json(_) => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct Entry {
    tokens: TokenStream,
}
impl Entry {
    fn new(str: &str) -> Result<Self, LexError> {
        match TokenStream::from_str(str) {
            Ok(n) => Ok(Entry { tokens: n }),
            Err(n) => Err(n),
        }
    }
}

impl Process for Entry {
    type Output = Built;

    fn next(self) -> Self::Output {
        todo!()
    }
}

#[derive(Debug)]
pub struct Built {
    tokens: TokenStream,
}
