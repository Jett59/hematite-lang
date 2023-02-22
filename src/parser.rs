use std::{error::Error, fmt::Display, iter::Peekable};

use crate::{
    ast::{
        AstNode, FunctionDefinition, IgnoreValue, ParameterDeclaration, Type, VariableDefinition,
    },
    lexer::{self, Token},
};

type TokenIterator<'lifetime> = Peekable<lexer::TokenIterator<'lifetime>>;

use Token::*;

#[derive(Clone, Debug)]
pub struct SyntaxError {
    message: String,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Syntax error: {}", self.message)
    }
}

impl Error for SyntaxError {}

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
            if end != None {
                token_iterator.next().unwrap();
            }
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

fn parse_variable_definition(token_iterator: &mut TokenIterator) -> ParsedItem {
    next_must_be!(token_iterator, Let);
    let mutable = if token_iterator.peek() == Some(&Mut) {
        token_iterator.next();
        true
    } else {
        false
    };
    let name = match token_iterator.next() {
        Some(token) => match token {
            Identifier(name) => name,
            _ => return Err(SyntaxError::unexpected_token(&token)),
        },
        None => return Err(SyntaxError::unexpected_end()),
    };
    next_must_be!(token_iterator, Colon);
    let variable_type = parse_type(token_iterator)?;
    next_must_be!(token_iterator, Equals);
    let value = parse_expression(token_iterator)?;
    next_must_be!(token_iterator, Semicolon);
    Ok(Box::new(VariableDefinition::new(
        mutable,
        name,
        variable_type,
        value,
    )))
}

fn parse_expression(token_iterator: &mut TokenIterator) -> ParsedItem {
    match token_iterator.next() {
        Some(token) => match token {
            Integer(value) => Ok(Box::new(value)),
            _ => Err(SyntaxError::unexpected_token(&token)),
        },
        None => Err(SyntaxError::unexpected_end()),
    }
}

fn parse_statement(token_iterator: &mut TokenIterator) -> ParsedItem {
    match token_iterator.peek() {
        Some(token) => match token {
            Let => parse_variable_definition(token_iterator),
            _ => {
                let expression = parse_expression(token_iterator)?;
                if token_iterator.peek() == Some(&Semicolon) {
                    token_iterator.next().unwrap();
                    Ok(Box::new(IgnoreValue::new(expression)))
                } else {
                    Ok(expression)
                }
            }
        },
        None => Err(SyntaxError::unexpected_end()),
    }
}

fn parse_block(token_iterator: &mut TokenIterator) -> ParsedItem {
    next_must_be!(token_iterator, LeftBrace);
    let statements = parse_repeated_item(token_iterator, parse_statement, Some(RightBrace))?;
    Ok(Box::new(statements))
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
    let name = if let Some(Identifier(name)) = token_iterator.peek() {
        Ok(name.clone())
    } else {
        Err(SyntaxError::unexpected(token_iterator.peek()))
    }?;
    token_iterator.next().unwrap();
    next_must_be!(token_iterator, LeftParen);
    let parameters = parse_repeated_item(
        token_iterator,
        parse_parameter_declaration,
        Some(RightParen),
    )?;
    next_must_be!(token_iterator, Arrow);
    let return_type = parse_type(token_iterator)?;
    let body = parse_block(token_iterator)?;
    Ok(Box::new(FunctionDefinition::new(
        name,
        parameters,
        return_type,
        body,
    )))
}

fn parse_program(token_iterator: &mut TokenIterator) -> ParsedItem {
    let children = parse_repeated_item(token_iterator, parse_global_item, None)?;
    Ok(Box::new(children))
}

pub fn parse(token_iterator: &mut TokenIterator) -> Result<Box<dyn AstNode>, SyntaxError> {
    parse_program(token_iterator)
}
