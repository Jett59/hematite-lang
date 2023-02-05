use proc_macro::TokenTree;
use quote::{format_ident, quote};

extern crate proc_macro;

/// Implements the `TokenParser` trait for a token which expects a fixed set of characters.
///
/// # Format
///
/// ```
/// EnumConstant: "exact match string"
/// ```
#[proc_macro]
pub fn exact_match_token(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut token_iterator = input.into_iter().peekable();
    let raw_enum_constant = match token_iterator.next().expect("Missing enum constant") {
        TokenTree::Ident(ident) => ident.to_string(),
        _ => panic!("Expected identifier for enum constant"),
    };
    let enum_constant_ident = format_ident!("{}", raw_enum_constant);
    let parser_struct_name = format_ident!("{}Parser", raw_enum_constant);
    assert!(
        token_iterator.next().expect("Missing ':'").to_string() == ":",
        "Expected ':' after enum constant"
    );
    let quoted_exact_match_string = match &token_iterator.next().expect("Missing string") {
        TokenTree::Literal(literal) => literal.to_string(),
        _ => panic!("Expected string literal"),
    };
    let exact_match_string = quoted_exact_match_string.trim_matches('"');
    quote! {
        struct #parser_struct_name {
            offset: usize,
        }

        impl #parser_struct_name {
            fn new() -> Self {
                Self { offset: 0 }
            }
        }

        impl TokenParser for #parser_struct_name {
            fn accept(&self, character: char) -> Option<Box<dyn TokenParser>> {
                if #exact_match_string.chars().nth(self.offset) == Some(character) {
                    Some(Box::new(#parser_struct_name {
                        offset: self.offset + 1,
                    }))
                }else {
                    None
                }
            }
            fn complete(&self) -> Option<Token> {
                if self.offset == #exact_match_string.len() {
                    Some(Token::#enum_constant_ident)
                } else {
                    None
                }
            }
        }
    }
    .into()
}
