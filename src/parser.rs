use std::iter::Peekable;

use crate::{
    ast::{AstNode, ParameterDeclaration, Program, Type},
    lexer::{self, Token},
};

type TokenIterator<'lifetime> = Peekable<lexer::TokenIterator<'lifetime>>;

use Token::*;

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
    fn unexpected(token: Option<&Token>) -> Self {
        if let Some(token) = token {
            Self::unexpected_token(token)
        } else {
            Self::unexpected_end()
        }
    }
}

macro_rules! next_must_be {
    ($token_iterator:ident, $expected:tt) => {
        match $token_iterator.next() {
            Some(token) => match token {
                $expected => {}
                _ => return Err(SyntaxError::unexpected_token(&token)),
            },
            _ => return Err(SyntaxError::unexpected_end()),
        }
    };
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
            token_iterator.next().unwrap();
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
    match token_iterator.peek() {
        Some(token) => match token {
            Function => parse_function(token_iterator),
            _ => Err(SyntaxError::unexpected_token(
                token_iterator.peek().unwrap(),
            )),
        },
        None => Err(SyntaxError::unexpected_end()),
    }
}

fn parse_type(token_iterator: &mut TokenIterator) -> ParsedItem {
    match token_iterator.next() {
        Some(token) => match token {
            I8 => Ok(Box::new(Type::I8)),
            I16 => Ok(Box::new(Type::I16)),
            I32 => Ok(Box::new(Type::I32)),
            I64 => Ok(Box::new(Type::I64)),
            Iptr => Ok(Box::new(Type::Iptr)),
            U8 => Ok(Box::new(Type::U8)),
            U16 => Ok(Box::new(Type::U16)),
            U32 => Ok(Box::new(Type::U32)),
            U64 => Ok(Box::new(Type::U64)),
            Uptr => Ok(Box::new(Type::Uptr)),
            F32 => Ok(Box::new(Type::F32)),
            F64 => Ok(Box::new(Type::F64)),
            Bool => Ok(Box::new(Type::Bool)),
            CharType => Ok(Box::new(Type::Char)),
            StringType => Ok(Box::new(Type::String)),
            _ => Err(SyntaxError::unexpected_token(&token)),
        },
        _ => Err(SyntaxError::unexpected_end()),
    }
}

fn parse_parameter_declaration(token_iterator: &mut TokenIterator) -> ParsedItem {
    let name = match token_iterator.next() {
        Some(token) => match token {
            Identifier(name) => name,
            _ => return Err(SyntaxError::unexpected_token(&token)),
        },
        _ => return Err(SyntaxError::unexpected_end()),
    };
    next_must_be!(token_iterator, Colon);
    let parameter_type = parse_type(token_iterator)?;
    if token_iterator.peek() == Some(&Comma) {
        token_iterator.next().unwrap();
    }
    Ok(Box::new(ParameterDeclaration::new(name, parameter_type)))
}

fn parse_function(token_iterator: &mut TokenIterator) -> ParsedItem {
    assert!(token_iterator.next() == Some(Token::Function));
    let name = if let Some(Identifier(name)) = token_iterator.next() {
        Ok(name)
    } else {
        Err(SyntaxError::unexpected(token_iterator.peek()))
    }?;
    next_must_be!(token_iterator, LeftParen);
    let parameters = parse_repeated_item(
        token_iterator,
        parse_parameter_declaration,
        Some(RightParen),
    )?;
    Err(SyntaxError::unexpected(token_iterator.peek()))
}

fn parse_program(token_iterator: &mut TokenIterator) -> ParsedItem {
    let children = parse_repeated_item(token_iterator, parse_global_item, None)?;
    Ok(Box::new(Program::new(children)))
}

pub fn parse(token_iterator: &mut TokenIterator) -> Result<Box<dyn AstNode>, SyntaxError> {
    parse_program(token_iterator)
}
