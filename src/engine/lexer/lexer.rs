use anyhow::{Context, Result};
use log::info;
use regex::Regex;

use super::token::{Token, TokenType, TokenKind};
use super::super::data::Data;

struct Lexer {
    contents: String,
    line: usize,
    col: usize,
    tokens: Vec<Token>,
    token_types: Vec<TokenType>,
}

impl Lexer {
    fn new(contents: &str, token_types: Vec<TokenType>) -> Self {
        let mut contents = contents.to_string();
        if contents.ends_with('\n') {
            contents.pop();
        }

        Self {
            contents,
            line: 1,
            col: 1,
            tokens: Vec::new(),
            token_types,
        }
    }

    fn lex(&mut self) -> Result<()> {
        // Find the next word (separated by whitespace, \t, \n, or \r) and try to regex it. Workd
        // should not be empty or contain only whitespace.
        let mut remove_symbols = 0;
        let mut word = String::new();
        let mut word_start_col = self.col;
        let word_start_line = self.line;
        for c in self.contents[0..].chars() {
            remove_symbols += 1;
            self.col += 1;
            if c == ' ' || c == '\t' || c == '\r' || c == '\n' {
                word_start_col += 1;
                if c == '\n' {
                    self.line += 1;
                    self.col = 1;
                    word_start_col = 1;
                }
                if word.is_empty() {
                    continue;
                }
                break;
            }
            word.push(c);
        }
        self.contents = self.contents[remove_symbols..].to_string();
        let mut found = false;

        for (i, token) in self.token_types.iter().enumerate() {
            let re = Regex::new(&token.regex).unwrap();
            if re.is_match(&word) {
                info!("Found token: {:?}", token);
                let mut data = Data::None;
                if token.type_ == TokenKind::Push {
                    let caps = re.captures(&word).unwrap();
                    data = Data::from_any(&caps[1]);
                }
                self.tokens.push(Token::new(i, data, word_start_line, word_start_col));
                found = true;
                break;
            }
        }

        if !found {
            return Err(anyhow::anyhow!(
                "No token found at line {}, col {}",
                self.line,
                self.col
            ))
            .context(format!(
                "No token found at line {}, col {}",
                self.line, self.col
            ));
        }

        Ok(())
    }
}

pub fn lex(contents: &str, token_types: Vec<TokenType>) -> Result<Vec<Token>> {
    let mut lexer = Lexer::new(contents, token_types);
    info!("Lexer created");

    while !lexer.contents.is_empty() {
        lexer.lex()?;
    }

    info!("Lexer finished");
    for token in &lexer.tokens {
        info!("{:?}", token);
    }

    Ok(lexer.tokens)
}
