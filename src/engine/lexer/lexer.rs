use anyhow::Result;
use log::{debug, info};
use regex::Regex;

use super::super::data::Data;
use super::token::{Token, TokenKind, TokenType};

struct Lexer {
    contents: String,
    raw_contents: String,
    file: String,
    line: usize,
    col: usize,
    tokens: Vec<Token>,
    token_types: Vec<TokenType>,
}

impl Lexer {
    fn new(contents: &str, token_types: Vec<TokenType>, file: String) -> Self {
        let mut contents = contents.to_string();
        if contents.ends_with('\n') {
            contents.pop();
        }
        contents.push(' ');

        Self {
            contents: contents.clone(),
            raw_contents: contents.clone(),
            file,
            line: 1,
            col: 1,
            tokens: Vec::new(),
            token_types,
        }
    }

    fn lex(&mut self) -> Result<()> {
        // Find the next word (separated by whitespace, \t, \n, or \r) and try to regex it. Word
        // should not be empty or contain only whitespace.
        let mut remove_symbols = 0;
        let mut word = String::new();
        let mut word_start_col = self.col;
        let word_start_line = self.line;
        let mut inside_quotes = false;
        for c in self.contents[0..].chars() {
            remove_symbols += 1;
            self.col += 1;
            if c == '"' {
                inside_quotes = !inside_quotes;
                continue;
            }

            if inside_quotes {
                word.push(c);
                continue;
            } else {
                if c == ' ' || c == '\t' || c == '\r' || c == '\n' {
                    word_start_col += 1;
                    if c == '\n' {
                        self.line += 1;
                        self.col = 1;
                        word_start_col = 1;
                    }
                    break;
                }
                word.push(c);
            }
        }
        if word.is_empty() {
            self.contents = self.contents[remove_symbols..].to_string();
            return Ok(());
        }
        self.contents = self.contents[remove_symbols..].to_string();
        debug!("Word: `{}`", word);
        let mut found = false;

        for (i, token) in self.token_types.iter().enumerate() {
            let re = Regex::new(&token.regex).unwrap();
            if re.is_match(&word) {
                debug!("Found token: {:?}", token);
                let mut data = Data::None;
                if token.type_ == TokenKind::Push {
                    let caps = re.captures(&word).unwrap();
                    data = Data::from_any(&caps[1]);
                }
                let line = self
                    .raw_contents
                    .lines()
                    .nth(word_start_line - 1)
                    .unwrap_or("");
                self.tokens.push(Token::new(
                    i,
                    data,
                    self.file.clone(),
                    word_start_line,
                    word_start_col,
                    line.to_string(),
                ));
                found = true;
                break;
            }
        }

        if !found {
            return Err(anyhow::anyhow!(
                "No token found at line {}, col {}: \"{}\"",
                self.line,
                self.col,
                word
            ));
        }

        Ok(())
    }
}

pub fn lex(contents: &str, token_types: Vec<TokenType>, file: String) -> Result<Vec<Token>> {
    let mut lexer = Lexer::new(contents, token_types, file);
    info!("Lexer created");

    while !lexer.contents.is_empty() {
        lexer.lex()?;
    }

    info!("Lexer finished");
    for token in &lexer.tokens {
        debug!("{:?}", token);
    }

    Ok(lexer.tokens)
}
