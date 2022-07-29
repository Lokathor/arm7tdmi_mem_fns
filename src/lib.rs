#![allow(unused)]

extern crate proc_macro;
use std::collections::{hash_map::Entry, HashMap};

use proc_macro::{Ident, Literal, TokenStream, TokenTree};

/// This holds all the `extern "C"` declarations for the assembly code that the
/// crate can provide.
#[warn(missing_docs)]
#[allow(dead_code)]
mod fn_declarations;

#[proc_macro]
pub fn generate_fns(token_stream: TokenStream) -> TokenStream {
  let mapping = parse_mapping(token_stream);

  "fn answer() -> u32 { 42 }".parse().unwrap()
}

fn parse_mapping(token_stream: TokenStream) -> HashMap<String, Literal> {
  let mut hm = HashMap::new();
  let mut tt_iter = token_stream.into_iter();
  loop {
    let ident = match tt_iter.next() {
      Some(TokenTree::Ident(ident)) => ident,
      None => break,
      other => panic!("Expected Some(Ident), got {other:?}"),
    };
    let _eq = match tt_iter.next() {
      Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => punct,
      other => panic!("Expected Some(Punct(=)), got {other:?}"),
    };
    let literal = match tt_iter.next() {
      Some(TokenTree::Literal(literal)) => literal,
      other => panic!("Expected Some(Literal), got {other:?}"),
    };
    let _comma = match tt_iter.next() {
      Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => punct,
      None => break,
      other => panic!("Expected Some(Punct(',')), got {other:?}"),
    };
    let ident_string = ident.to_string();
    match hm.entry(ident_string) {
      Entry::Occupied(_) => panic!("Identifier {ident} was specified twice!"),
      Entry::Vacant(ve) => ve.insert(literal),
    };
  }
  hm
}
