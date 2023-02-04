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
}

trait TokenParser {
    fn parse(&self, input: &mut dyn Iterator<Item = char>) -> Option<(usize, Token)>;
}

pub struct TokenIterator<'base_iterator> {
    base_iterator: &'base_iterator mut dyn Iterator<Item = char>,
    prematurely_read_characters: String,
}

impl Iterator for TokenIterator<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut newly_read_characters = String::new();
        let mut character_iterator =
            self.prematurely_read_characters
                .chars()
                .chain(self.base_iterator.map(|character| {
                    newly_read_characters.push(character);
                    character
                }));
        // We have to define this as a local to avoid compile errors.
        let token_parsers: &[&dyn TokenParser] = &[];
        for token_parser in token_parsers {
            if let Some((token_length, token)) = token_parser.parse(&mut character_iterator) {
                self.prematurely_read_characters = (self.prematurely_read_characters.clone()
                    + newly_read_characters.as_str())[token_length..]
                    .to_string();
                return Some(token);
            }
        }
        None
    }
}

pub fn tokenize(input: &mut dyn Iterator<Item = char>) -> TokenIterator {
    TokenIterator {
        base_iterator: input,
        prematurely_read_characters: String::new(),
    }
}
