use std::collections::HashMap;

use crate::error::ParseError;
use crate::tokenizer::TokenKind;
use crate::{parse::Parse, token_stream::TokenStream, tokenizer::Tokenizer};

#[derive(Debug, Clone, PartialEq)]
pub enum JsonElement {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<JsonElement>),
    Object(HashMap<String, JsonElement>),
    Null,
}

pub struct Json {
    pub root: JsonElement,
}

impl Json {
    fn parse_tokens(mut tokens: TokenStream) -> Result<JsonElement, ParseError> {
        fn parse_value(tokens: &mut TokenStream) -> Result<JsonElement, ParseError> {
            use TokenKind::*;

            let tk = tokens.current();

            match tk.kind {
                String => {
                    let t = tokens.advance();
                    Ok(JsonElement::String(t.lexeme))
                }
                Number => {
                    let t = tokens.advance();
                    let n = t
                        .lexeme
                        .parse::<f64>()
                        .map_err(|_e| ParseError::new(format!("Invalid number '{}'", t.lexeme)))?;
                    Ok(JsonElement::Number(n))
                }
                Boolean => {
                    let t = tokens.advance();
                    let b = match t.lexeme.as_str() {
                        "true" => true,
                        "false" => false,
                        other => {
                            return Err(ParseError::new(format!(
                                "Invalid boolean literal '{}'",
                                other
                            )));
                        }
                    };
                    Ok(JsonElement::Boolean(b))
                }
                Null => {
                    tokens.advance();
                    Ok(JsonElement::Null)
                }
                LeftBrace => parse_object(tokens),
                LeftBracket => parse_array(tokens),
                _ => Err(ParseError::new(format!("Unexpected token: {:?}", tk.kind))),
            }
        }

        fn parse_object(tokens: &mut TokenStream) -> Result<JsonElement, ParseError> {
            use TokenKind::*;

            let start = tokens.current();
            if start.kind != LeftBrace {
                return Err(ParseError::new("Expected '{' at start of object"));
            }
            tokens.advance();

            let mut map = HashMap::new();

            if tokens.current().kind == RightBrace {
                tokens.advance();
                return Ok(JsonElement::Object(map));
            }

            loop {
                let key_token = tokens.current();
                if key_token.kind != String {
                    return Err(ParseError::new(format!(
                        "Expected string key in object, found {:?}",
                        key_token.kind
                    )));
                }
                let key = tokens.advance().lexeme;

                if tokens.current().kind != Colon {
                    return Err(ParseError::new("Expected ':' after object key"));
                }
                tokens.advance();

                let value = parse_value(tokens)?;
                map.insert(key, value);

                match tokens.current().kind {
                    Comma => {
                        tokens.advance();
                        continue;
                    }
                    RightBrace => {
                        tokens.advance();
                        break;
                    }
                    other => {
                        return Err(ParseError::new(format!(
                            "Expected ',' or '}}' in object, found {:?}",
                            other
                        )));
                    }
                }
            }

            Ok(JsonElement::Object(map))
        }

        fn parse_array(tokens: &mut TokenStream) -> Result<JsonElement, ParseError> {
            use TokenKind::*;

            let start = tokens.current();
            if start.kind != LeftBracket {
                return Err(ParseError::new("Expected '[' at start of array"));
            }
            tokens.advance();

            let mut items = Vec::new();

            if tokens.current().kind == RightBracket {
                tokens.advance();
                return Ok(JsonElement::Array(items));
            }

            loop {
                let value = parse_value(tokens)?;
                items.push(value);

                match tokens.current().kind {
                    Comma => {
                        tokens.advance();
                        continue;
                    }
                    RightBracket => {
                        tokens.advance();
                        break;
                    }
                    other => {
                        return Err(ParseError::new(format!(
                            "Expected ',' or ']' in array, found {:?}",
                            other
                        )));
                    }
                }
            }

            Ok(JsonElement::Array(items))
        }

        let value = parse_value(&mut tokens)?;
        Ok(value)
    }

    fn _parse(source: impl Into<String>) -> Result<Self, ParseError> {
        let mut tokenizer = Tokenizer::new(source.into());
        let tokens = tokenizer.scan_tokens()?;
        let root = Self::parse_tokens(tokens)?;

        Ok(Json { root })
    }
}

impl Parse for Json {
    fn parse(source: impl Into<String>) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        Self::_parse(source)
    }
}
