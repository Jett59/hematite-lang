use std::iter::Peekable;

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    MacroCall(String),
    Integer(i128),
    Float(f64),
    String(String),
    Char(char),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Colon,
    Semicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Arrow,
    Function,
    Let,
    If,
    Else,

    Error(String),
}

trait TokenParser {
    fn accept(&self, character: char) -> Option<Box<dyn TokenParser>>;
    /// Return `Some(Token)` if complete, otherwise None.
    /// This function will be called if this token parser returns false in the last round where there are any possibilities left.
    fn complete(&self) -> Option<Token>;
}

helper_macros::exact_match_token! {Arrow: "->"}

pub struct TokenIterator<'base_iterator> {
    base_iterator: Peekable<&'base_iterator mut dyn Iterator<Item = char>>,
}

impl Iterator for TokenIterator<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.base_iterator.peek().is_some()
            && self.base_iterator.peek().unwrap().is_whitespace()
        {
            self.base_iterator.next();
        }
        let mut possibilities: Vec<Box<dyn TokenParser>> = vec![Box::new(ArrowParser::new())];
        let mut characters_read_so_far = String::new();
        while let Some(next_character) = self.base_iterator.peek() {
            let new_possibilities = possibilities
                .iter()
                .filter_map(|possibility| possibility.accept(*next_character))
                .collect::<Vec<_>>();
            if new_possibilities.is_empty() {
                // This means that we have read a complete token or the input is invalid.
                let mut completed_tokens = possibilities
                    .iter()
                    .filter_map(|possibility| possibility.complete());
                // We just take the first one.
                // This should mean (assuming I'm right that they keep their order) that placing keywords above identifier *should* work.
                if let Some(completed_token) = completed_tokens.next() {
                    return Some(completed_token);
                } else {
                    return Some(Token::Error(format!(
                        "Invalid token: {characters_read_so_far}{next_character}",
                    )));
                }
            } else {
                possibilities = new_possibilities;
                characters_read_so_far.push(*next_character);
                self.base_iterator.next().unwrap();
            }
        }
        None
    }
}

pub fn tokenize(input: &mut dyn Iterator<Item = char>) -> TokenIterator {
    TokenIterator {
        base_iterator: input.peekable(),
    }
}
