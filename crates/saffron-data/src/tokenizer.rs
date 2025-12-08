use std::fmt::{self, Display};

use crate::error::ParseError;
use crate::token_stream::TokenStream;

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
    pub span: Span,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.kind, self.lexeme)
    }
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize, column: usize, span: Span) -> Self {
        Token {
            kind,
            lexeme,
            line,
            column,
            span,
        }
    }

    pub fn new_synthetic(lexeme: &str) -> Self {
        Token {
            kind: TokenKind::Identifier,
            lexeme: lexeme.to_string(),
            line: 0,
            column: 0,
            span: Span(0, 1),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Span(pub usize, pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    String,
    Number,
    Boolean,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Null,
    Identifier,
    EndOfFile,
}

pub struct Tokenizer {
    pub(crate) source: Vec<char>,
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) start: usize,
    pub(crate) length: usize,
    pub(crate) tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(source: String) -> Self {
        Tokenizer {
            source: source.chars().collect(),
            line: 1,
            column: 1,
            start: 0,
            length: 0,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<TokenStream, ParseError> {
        while !self.is_at_end() {
            self.scan_token()?;
        }

        self.make_token_with_lexeme(TokenKind::EndOfFile, String::from("\0"));

        Ok(TokenStream::new(self.tokens.clone()))
    }

    fn sync_cursors(&mut self) {
        self.start += self.length;
        self.length = 0;
    }

    fn scan_token(&mut self) -> Result<(), ParseError> {
        self.sync_cursors();

        let c: char = self.advance().unwrap_or('\0');

        match c {
            '\n' => {
                self.column = 1;
                self.line += 1;
            }
            ' ' | '\t' => {}
            '\r' => {}
            '[' => self.make_token(TokenKind::LeftBracket),
            ']' => self.make_token(TokenKind::RightBracket),
            '{' => self.make_token(TokenKind::LeftBrace),
            '}' => self.make_token(TokenKind::RightBrace),

            ',' => self.make_token(TokenKind::Comma),
            ':' => self.make_token(TokenKind::Colon),

            '"' => self.string('"')?,
            '\'' => self.string('\'')?,

            '-' => {
                if self.is_digit(self.peek()) {
                    self.number();
                } else {
                    return Err(ParseError::new(format!(
                        "Invalid character '{}' at line {}",
                        c, self.line
                    )));
                }
            }

            _ => match c {
                _ if self.is_digit(c) => {
                    self.number();
                }
                _ if self.is_alpha(c) => {
                    self.identifier_or_keyword();
                }
                _ => {
                    return Err(ParseError::new(format!(
                        "Invalid character '{}' at line {}",
                        c, self.line
                    )));
                }
            },
        }
        Ok(())
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.check('.') && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let lexeme: String = self.source[self.start..self.start + self.length]
            .iter()
            .collect();

        self.make_token_with_lexeme(TokenKind::Number, lexeme);
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_digit(c) || self.is_alpha(c)
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_lowercase() || c.is_ascii_uppercase()
    }

    fn string(&mut self, end: char) -> Result<(), ParseError> {
        let mut value = String::new();
        let mut escaped = false;

        while !self.is_at_end() {
            let c = self.peek();

            if escaped {
                let escape_char = match c {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '"' => '"',
                    '\'' => '\'',
                    other => other,
                };
                value.push(escape_char);
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == end {
                break;
            } else {
                value.push(c);
            }

            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(ParseError::new(format!(
                "Unterminated string at line {}.",
                self.line
            )));
        }

        self.advance();

        self.make_token_with_lexeme(TokenKind::String, value);

        Ok(())
    }

    fn identifier_or_keyword(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let lexeme: String = self.source[self.start..self.start + self.length]
            .iter()
            .collect();

        match lexeme.as_str() {
            "true" | "false" => self.make_token_with_lexeme(TokenKind::Boolean, lexeme),
            "null" => self.make_token_with_lexeme(TokenKind::Null, lexeme),
            _ => self.make_token_with_lexeme(TokenKind::Identifier, lexeme),
        }
    }

    fn make_token(&mut self, kind: TokenKind) {
        let lexeme: String = self.source[self.start..self.start + self.length]
            .iter()
            .collect();
        self.make_token_with_lexeme(kind, lexeme);
    }

    fn make_token_with_lexeme(&mut self, kind: TokenKind, lexeme: String) {
        let span = Span(self.start, self.start + self.length);
        let token = Token::new(kind, lexeme, self.line, self.column, span);
        self.tokens.push(token);
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        let c = self.source.get(self.start + self.length).cloned();
        self.length += 1;
        self.column += 1;
        c
    }

    fn check(&self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        c.eq(&self.peek())
    }

    fn peek(&self) -> char {
        self.source
            .get(self.start + self.length)
            .cloned()
            .unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source
            .get(self.start + self.length + 1)
            .cloned()
            .unwrap_or('\0')
    }

    fn is_at_end(&self) -> bool {
        self.start + self.length >= self.source.len()
    }
}
