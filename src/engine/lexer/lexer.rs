use anyhow::Result;
use log::{debug, info};
use regex::Regex;

use super::super::{data::Data, file::File};
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

    fn lex(&mut self, imported_files: &mut Vec<String>) -> Result<()> {
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
                let caps = re.captures(&word).unwrap();
                if caps.len() > 1 {
                    data = Data::from_any(&caps[1]);
                }
                if token.type_ == TokenKind::Import {
                    let mut file_name = data.to_string();
                    info!("Importing file: {}", file_name);
                    if !file_name.ends_with(".seal") {
                        file_name.push_str(".seal");
                    }
                    if !imported_files.contains(&file_name) {
                        info!("Importing file: {}", file_name);
                        imported_files.push(file_name.clone());
                        let mut main_file = File::new(file_name.clone(), file_name)?;
                        main_file.read()?;
                        let mut tokens = lex(
                            main_file.contents.as_str(),
                            self.token_types.clone(),
                            main_file.path.clone(),
                            imported_files,
                        )?;
                        self.tokens.append(&mut tokens);
                    } else {
                        info!("File already imported: {}", file_name);
                    }
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

    pub fn after_lex(&mut self) -> Result<()> {
        info!("Starting after-lexing");

        let tokens = self.tokens.clone();
        for (i, token) in &mut self.tokens.iter_mut().enumerate() {
            let token_type = token.get_type(self.token_types.clone())?.type_;
            debug!("Token: {:?}", token);
            if token_type == TokenKind::If {
                // Go through all the tokens and find the matching end token
                let mut end_token = None;
                let mut depth = 0;
                let tokens = tokens.clone()[i + 1..].to_vec();
                for (i, t) in tokens.iter().enumerate() {
                    let token_type = t.get_type(self.token_types.clone())?;
                    debug!(
                        "Subtoken: {:?}, type: {:?}",
                        t,
                        t.get_type(self.token_types.clone())
                    );
                    if token_type.type_ == TokenKind::EndIf {
                        if depth == 0 {
                            end_token = Some(i);
                            break;
                        } else {
                            depth -= 1;
                        }
                    } else if token_type.type_ == TokenKind::If {
                        depth += 1;
                    }
                }
                if end_token.is_none() {
                    return Err(anyhow::anyhow!(
                        "No matching end token found for token at {}:{}",
                        token.line,
                        token.col
                    ));
                }
                token.data = Data::from_int(end_token.unwrap() as i32);
            } else if token_type == TokenKind::Do {
                // Go through all the tokens and find the matching end token
                let mut end_token = None;
                let mut depth = 0;
                let tokens = tokens.clone()[i + 1..].to_vec();
                for (i, t) in tokens.iter().enumerate() {
                    let token_type = t.get_type(self.token_types.clone())?;
                    debug!(
                        "Subtoken: {:?}, type: {:?}",
                        t,
                        t.get_type(self.token_types.clone())
                    );
                    if token_type.type_ == TokenKind::End {
                        if depth == 0 {
                            end_token = Some(i);
                            break;
                        } else {
                            depth -= 1;
                        }
                    } else if token_type.type_ == TokenKind::While {
                        depth += 1;
                    }
                }
                if end_token.is_none() {
                    return Err(anyhow::anyhow!(
                        "No matching end token found for token at {}:{}",
                        token.line,
                        token.col
                    ));
                }
                token.data = Data::from_int(end_token.unwrap() as i32);
            } else if token_type == TokenKind::End {
                // Go back through ass the tokens and find a while token
                let mut while_token = None;
                let mut depth = 0;
                let mut tokens = tokens.clone()[..i].to_vec();
                tokens.reverse();
                for (i, t) in tokens.iter().enumerate() {
                    let token_type = t.get_type(self.token_types.clone())?;
                    let token_kind = token_type.type_.clone();
                    debug!("Subtoken: {:?}, type: {:?}", "", token_type.type_);
                    if token_type.type_ == TokenKind::While {
                        if depth == 0 {
                            while_token = Some(i);
                            break;
                        } else {
                            depth -= 1;
                        }
                    } else if token_kind == TokenKind::End {
                        depth += 1;
                    }
                }
                if while_token.is_none() {
                    return Err(anyhow::anyhow!(
                        "No matching end token found for token at {}:{}",
                        token.line,
                        token.col
                    ));
                }
                token.data = Data::from_int(while_token.unwrap() as i32)
            } else if token_type == TokenKind::Proc {
                // Go through all the tokens and find the matching ret token
                let mut ret_token = None;
                let mut depth = 0;
                let tokens = tokens.clone()[i + 1..].to_vec();
                for (i, t) in tokens.iter().enumerate() {
                    let token_type = t.get_type(self.token_types.clone())?;
                    debug!(
                        "Subtoken: {:?}, type: {:?}",
                        t,
                        t.get_type(self.token_types.clone())
                    );
                    if token_type.type_ == TokenKind::ProcRet {
                        if depth == 0 {
                            ret_token = Some(i);
                            break;
                        } else {
                            depth -= 1;
                        }
                    } else if token_type.type_ == TokenKind::Proc {
                        depth += 1;
                    }
                }
                if ret_token.is_none() {
                    return Err(anyhow::anyhow!(
                        "No matching ret token found for token at {}:{}",
                        token.line,
                        token.col
                    ));
                }
                token.data = Data::from_int(ret_token.unwrap() as i32);
            }
        }

        info!("After-lexing complete");
        Ok(())
    }
}

pub fn lex(
    contents: &str,
    token_types: Vec<TokenType>,
    file: String,
    imported_files: &mut Vec<String>,
) -> Result<Vec<Token>> {
    let mut lexer = Lexer::new(contents, token_types, file);
    info!("Lexer created");

    while !lexer.contents.is_empty() {
        lexer.lex(imported_files)?;
    }

    lexer.after_lex()?;

    info!("Lexer finished");
    for token in &lexer.tokens {
        debug!("{:?}", token);
    }

    Ok(lexer.tokens)
}
