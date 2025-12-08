use crate::tokenizer::{Span, Token, TokenKind};

#[derive(Default)]
pub struct TokenStream {
    tokens: Vec<Token>,
    position: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn previous(&self) -> Token {
        if self.tokens.is_empty() {
            return Token::new(TokenKind::EndOfFile, "\0".into(), 0, 0, Span(0, 0));
        }

        if self.position == 0 {
            return self.tokens[0].clone();
        }

        let pos = if self.position > self.tokens.len() {
            self.tokens.len() - 1
        } else {
            self.position - 1
        };

        self.tokens[pos].clone()
    }

    pub fn current(&self) -> Token {
        if self.tokens.is_empty() {
            return Token::new(TokenKind::EndOfFile, "\0".into(), 0, 0, Span(0, 0));
        }

        if self.position >= self.tokens.len() {
            return self.tokens.last().unwrap().clone();
        }

        self.tokens[self.position].clone()
    }

    pub fn advance(&mut self) -> Token {
        self.position += 1;
        self.previous()
    }

    pub fn look_ahead(&self, k: usize) -> Token {
        if self.tokens.is_empty() {
            return Token::new(TokenKind::EndOfFile, "\0".into(), 0, 0, Span(0, 0));
        }

        if self.position + k >= self.tokens.len() {
            return self.tokens.last().unwrap().clone();
        }

        self.tokens[self.position + k].clone()
    }

    pub fn backtrack(&mut self) {
        self.position -= 1;
    }
}

impl From<TokenStream> for Vec<Token> {
    fn from(value: TokenStream) -> Self {
        value.tokens
    }
}
