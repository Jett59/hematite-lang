use std::iter::Peekable;

use crate::{
    ast::{AstNode, Program},
    lexer::{self, Token},
};

type TokenIterator<'lifetime> = Peekable<lexer::TokenIterator<'lifetime>>;

#[derive(Clone, Debug)]
pub struct SyntaxError {
    message: String,
}

impl SyntaxError {
    fn unexpected_token(token: &Token) -> Self {
        Self {
            message: format!("Unexpected token: {token}"),
        }
    }
    fn unexpected_end() -> Self {
        Self {
            message: "Unexpected end of input".to_string(),
        }
    }
}

type ParsedItem = Result<Box<dyn AstNode>, SyntaxError>;

fn parse_repeated_item(
    token_iterator: &mut TokenIterator,
    parser_function: impl Fn(&mut TokenIterator) -> ParsedItem,
    end: Option<Token>,
) -> Result<Vec<Box<dyn AstNode>>, SyntaxError> {
    let mut items = Vec::new();

    loop {
        let token = token_iterator.peek();
        if token == end.as_ref() {
            return Ok(items);
        } else if token.is_none() {
            return Err(SyntaxError::unexpected_end());
        } else {
            let item = parser_function(token_iterator)?;
            items.push(item);
        }
    }
}

fn parse_global_item(token_iterator: &mut TokenIterator) -> ParsedItem {
    Err(SyntaxError::unexpected_token(
        token_iterator.peek().unwrap(),
    ))
}

fn parse_program(token_iterator: &mut TokenIterator) -> ParsedItem {
    let children = parse_repeated_item(token_iterator, parse_global_item, None)?;
    Ok(Box::new(Program::new(children)))
}

pub fn parse(token_iterator: &mut TokenIterator) -> Result<Box<dyn AstNode>, SyntaxError> {
    parse_program(token_iterator)
}
