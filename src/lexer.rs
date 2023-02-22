use std::{
    fmt::{self, Display, Formatter},
    iter::Peekable,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    MacroCall(String),
    Integer(i128),
    Float(f64),
    StringLiteral(String),
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
    Equals,
    Function,
    Let,
    Mut,
    If,
    Else,
    I8,
    I16,
    I32,
    I64,
    Iptr,
    U8,
    U16,
    U32,
    U64,
    Uptr,
    F32,
    F64,
    Bool,
    CharType,
    StringType,

    Error(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Identifier(s) => write!(f, "'{s}'"),
            Token::MacroCall(s) => write!(f, "'{s}!'"),
            Token::Integer(i) => write!(f, "'{i}'"),
            Token::Float(value) => write!(f, "'{value}'"),
            Token::StringLiteral(s) => write!(f, "'\"{s}\"'"),
            Token::Char(c) => write!(f, "\"'{c}'\""),
            Token::LeftParen => write!(f, "'('"),
            Token::RightParen => write!(f, "')'"),
            Token::LeftBrace => write!(f, "'{{'"),
            Token::RightBrace => write!(f, "'}}'"),
            Token::LeftBracket => write!(f, "'['"),
            Token::RightBracket => write!(f, "']'"),
            Token::Comma => write!(f, "','"),
            Token::Dot => write!(f, "'.'"),
            Token::Colon => write!(f, "':'"),
            Token::Semicolon => write!(f, "';'"),
            Token::Plus => write!(f, "'+'"),
            Token::Minus => write!(f, "'-'"),
            Token::Star => write!(f, "'*'"),
            Token::Slash => write!(f, "'/'"),
            Token::Percent => write!(f, "'%'"),
            Token::Arrow => write!(f, "'->'"),
            Token::Equals => write!(f, "'='"),
            Token::Function => write!(f, "'function'"),
            Token::Let => write!(f, "'let'"),
            Token::Mut => write!(f, "'mut'"),
            Token::If => write!(f, "'if'"),
            Token::Else => write!(f, "'else'"),
            Token::I8 => write!(f, "'i8'"),
            Token::I16 => write!(f, "'i16'"),
            Token::I32 => write!(f, "'i32'"),
            Token::I64 => write!(f, "'i64'"),
            Token::Iptr => write!(f, "'iptr'"),
            Token::U8 => write!(f, "'u8'"),
            Token::U16 => write!(f, "'u16'"),
            Token::U32 => write!(f, "'u32'"),
            Token::U64 => write!(f, "'u64'"),
            Token::Uptr => write!(f, "'uptr'"),
            Token::F32 => write!(f, "'f32'"),
            Token::F64 => write!(f, "'f64'"),
            Token::Bool => write!(f, "'bool'"),
            Token::CharType => write!(f, "'char'"),
            Token::StringType => write!(f, "'string'"),
            Token::Error(s) => write!(f, "'{s}'"),
        }
    }
}

trait TokenParser {
    fn accept(&self, character: char) -> Option<Box<dyn TokenParser>>;
    /// Return `Some(Token)` if complete, otherwise None.
    /// This function will be called if this token parser returns false in the last round where there are any possibilities left.
    fn complete(&self) -> Option<Token>;
}

struct IdentifierParser {
    so_far: String,
}

impl IdentifierParser {
    fn new() -> Self {
        Self {
            so_far: String::new(),
        }
    }
}

impl TokenParser for IdentifierParser {
    fn accept(&self, character: char) -> Option<Box<dyn TokenParser>> {
        if character.is_alphabetic()
            || character == '_'
            || (!self.so_far.is_empty() && character.is_ascii_digit())
        {
            Some(Box::new(IdentifierParser {
                so_far: format!("{}{}", self.so_far, character),
            }))
        } else {
            None
        }
    }
    fn complete(&self) -> Option<Token> {
        Some(Token::Identifier(self.so_far.clone()))
    }
}

struct MacroCallParser {
    so_far: String,
    found_bang: bool,
}

impl MacroCallParser {
    fn new() -> Self {
        Self {
            so_far: String::new(),
            found_bang: false,
        }
    }
}

impl TokenParser for MacroCallParser {
    fn accept(&self, character: char) -> Option<Box<dyn TokenParser>> {
        if self.found_bang {
            None
        } else if character.is_alphanumeric() || character == '_' {
            Some(Box::new(MacroCallParser {
                so_far: format!("{}{}", self.so_far, character),
                found_bang: false,
            }))
        } else if character == '!' {
            Some(Box::new(MacroCallParser {
                so_far: self.so_far.clone(),
                found_bang: true,
            }))
        } else {
            None
        }
    }
    fn complete(&self) -> Option<Token> {
        if self.found_bang {
            Some(Token::MacroCall(self.so_far.clone()))
        } else {
            None
        }
    }
}

struct IntegerParser {
    so_far: String,
}

impl IntegerParser {
    fn new() -> Self {
        Self {
            so_far: String::new(),
        }
    }
}

impl TokenParser for IntegerParser {
    fn accept(&self, character: char) -> Option<Box<dyn TokenParser>> {
        if character.is_numeric() {
            Some(Box::new(IntegerParser {
                so_far: format!("{}{}", self.so_far, character),
            }))
        } else {
            None
        }
    }
    fn complete(&self) -> Option<Token> {
        Some(Token::Integer(self.so_far.parse().unwrap()))
    }
}

struct FloatParser {
    so_far: String,
    found_dot: bool,
}

impl FloatParser {
    fn new() -> Self {
        Self {
            so_far: String::new(),
            found_dot: false,
        }
    }
}

impl TokenParser for FloatParser {
    fn accept(&self, character: char) -> Option<Box<dyn TokenParser>> {
        if character.is_numeric() {
            Some(Box::new(FloatParser {
                so_far: format!("{}{}", self.so_far, character),
                found_dot: self.found_dot,
            }))
        } else if character == '.' && !self.found_dot {
            Some(Box::new(FloatParser {
                so_far: format!("{}{}", self.so_far, character),
                found_dot: true,
            }))
        } else {
            None
        }
    }
    fn complete(&self) -> Option<Token> {
        if self.found_dot {
            Some(Token::Float(self.so_far.parse().unwrap()))
        } else {
            None
        }
    }
}

struct StringParser {
    so_far: String,
    found_initial_quote: bool,
    found_terminal_quote: bool,
    next_character_is_escaped: bool,
}

impl StringParser {
    fn new() -> Self {
        Self {
            so_far: String::new(),
            found_initial_quote: false,
            found_terminal_quote: false,
            next_character_is_escaped: false,
        }
    }
}

impl TokenParser for StringParser {
    fn accept(&self, character: char) -> Option<Box<dyn TokenParser>> {
        if self.found_terminal_quote {
            return None;
        }
        if !self.found_initial_quote {
            if character == '"' {
                Some(Box::new(StringParser {
                    so_far: self.so_far.clone(),
                    found_initial_quote: true,
                    found_terminal_quote: false,
                    next_character_is_escaped: false,
                }))
            } else {
                None
            }
        } else if character == '"' && !self.next_character_is_escaped {
            Some(Box::new(StringParser {
                so_far: self.so_far.clone(),
                found_initial_quote: true,
                found_terminal_quote: true,
                next_character_is_escaped: false,
            }))
        } else {
            Some(Box::new(StringParser {
                so_far: format!("{}{}", self.so_far, character),
                found_initial_quote: true,
                found_terminal_quote: false,
                next_character_is_escaped: character == '\\',
            }))
        }
    }
    fn complete(&self) -> Option<Token> {
        if self.found_terminal_quote {
            Some(Token::StringLiteral(self.so_far.clone()))
        } else {
            None
        }
    }
}

helper_macros::exact_match_token! {LeftParen: "("}
helper_macros::exact_match_token! {RightParen: ")"}
helper_macros::exact_match_token! {LeftBrace: "{"}
helper_macros::exact_match_token! {RightBrace: "}"}
helper_macros::exact_match_token! {LeftBracket: "["}
helper_macros::exact_match_token! {RightBracket: "]"}
helper_macros::exact_match_token! {Comma: ","}
helper_macros::exact_match_token! {Dot: "."}
helper_macros::exact_match_token! {Colon: ":"}
helper_macros::exact_match_token! {Semicolon: ";"}
helper_macros::exact_match_token! {Plus: "+"}
helper_macros::exact_match_token! {Minus: "-"}
helper_macros::exact_match_token! {Star: "*"}
helper_macros::exact_match_token! {Slash: "/"}
helper_macros::exact_match_token! {Percent: "%"}
helper_macros::exact_match_token! {Arrow: "->"}
helper_macros::exact_match_token! {Equals: "="}
helper_macros::exact_match_token! {Function: "function"}
helper_macros::exact_match_token! {Let: "let"}
helper_macros::exact_match_token! {Mut: "mut"}
helper_macros::exact_match_token! {If: "if"}
helper_macros::exact_match_token! {Else: "else"}
helper_macros::exact_match_token! {I8: "i8"}
helper_macros::exact_match_token! {I16: "i16"}
helper_macros::exact_match_token! {I32: "i32"}
helper_macros::exact_match_token! {I64: "i64"}
helper_macros::exact_match_token! {Iptr: "iptr"}
helper_macros::exact_match_token! {U8: "u8"}
helper_macros::exact_match_token! {U16: "u16"}
helper_macros::exact_match_token! {U32: "u32"}
helper_macros::exact_match_token! {U64: "u64"}
helper_macros::exact_match_token! {Uptr: "uptr"}
helper_macros::exact_match_token! {F32: "f32"}
helper_macros::exact_match_token! {F64: "f64"}
helper_macros::exact_match_token! {Bool: "bool"}
helper_macros::exact_match_token! {CharType: "char"}
helper_macros::exact_match_token! {StringType: "string"}

pub struct TokenIterator<'base_iterator> {
    base_iterator: Peekable<&'base_iterator mut dyn Iterator<Item = char>>,
    found_invalid_token: bool,
}

impl Iterator for TokenIterator<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.found_invalid_token {
            return None;
        }
        while self
            .base_iterator
            .peek()
            .filter(|character| character.is_whitespace())
            .is_some()
        {
            self.base_iterator.next();
        }
        let mut possibilities: Vec<Box<dyn TokenParser>> = vec![
            Box::new(FunctionParser::new()),
            Box::new(LetParser::new()),
            Box::new(MutParser::new()),
            Box::new(IfParser::new()),
            Box::new(ElseParser::new()),
            Box::new(I8Parser::new()),
            Box::new(I16Parser::new()),
            Box::new(I32Parser::new()),
            Box::new(I64Parser::new()),
            Box::new(IptrParser::new()),
            Box::new(U8Parser::new()),
            Box::new(U16Parser::new()),
            Box::new(U32Parser::new()),
            Box::new(U64Parser::new()),
            Box::new(UptrParser::new()),
            Box::new(F32Parser::new()),
            Box::new(F64Parser::new()),
            Box::new(BoolParser::new()),
            Box::new(CharTypeParser::new()),
            Box::new(StringTypeParser::new()),
            Box::new(IdentifierParser::new()),
            Box::new(MacroCallParser::new()),
            Box::new(FloatParser::new()),
            Box::new(IntegerParser::new()),
            Box::new(StringParser::new()),
            Box::new(LeftParenParser::new()),
            Box::new(RightParenParser::new()),
            Box::new(LeftBraceParser::new()),
            Box::new(RightBraceParser::new()),
            Box::new(LeftBracketParser::new()),
            Box::new(RightBracketParser::new()),
            Box::new(CommaParser::new()),
            Box::new(DotParser::new()),
            Box::new(ColonParser::new()),
            Box::new(SemicolonParser::new()),
            Box::new(PlusParser::new()),
            Box::new(MinusParser::new()),
            Box::new(StarParser::new()),
            Box::new(SlashParser::new()),
            Box::new(PercentParser::new()),
            Box::new(ArrowParser::new()),
            Box::new(EqualsParser::new()),
        ];
        let mut characters_read_so_far = String::new();
        while let Some(next_character) = self.base_iterator.peek() {
            let new_possibilities = possibilities
                .iter()
                .filter_map(|possibility| possibility.accept(*next_character))
                .collect::<Vec<_>>();
            if new_possibilities.is_empty() {
                // This means that we have read a complete token or the input is invalid.
                if characters_read_so_far.is_empty() {
                    self.found_invalid_token = true;
                    return Some(Token::Error(
                        format!("Invalid character: {next_character}",),
                    ));
                }
                let mut completed_tokens = possibilities
                    .iter()
                    .filter_map(|possibility| possibility.complete());
                // We just take the first one.
                // This should mean (assuming I'm right that they keep their order) that placing keywords above identifier *should* work.
                if let Some(completed_token) = completed_tokens.next() {
                    return Some(completed_token);
                } else {
                    self.found_invalid_token = true;
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
        found_invalid_token: false,
    }
}
