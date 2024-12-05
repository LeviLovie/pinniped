use anyhow::Result;
use colored::Colorize;
use log::{info, debug};

use super::data::Data;
use super::file::File;
use super::lexer::{
    lexer::lex,
    token::{Token, TokenType, TokenKind},
};
use super::stack::*;
use super::mark::*;
use crate::args::Args;

pub struct Machine {
    args: Args,
    stack: Stack,
    token_types: Vec<TokenType>,
    main_file: Option<File>,
    tokens: Vec<Token>,
    marks: MarkList,
    pc: usize,
}

impl Machine {
    pub fn new(args: Args) -> Self {
        Self {
            args,
            stack: Stack::new(),
            token_types: Vec::new(),
            main_file: None,
            tokens: Vec::new(),
            marks: MarkList::new(),
            pc: 0,
        }
    }

    pub fn register_tokens(&mut self, tokens: Vec<TokenType>) {
        self.token_types.extend(tokens);
        info!("Tokens registered");
    }

    pub fn preprocess(&mut self) -> Result<()> {
        info!("Preprocessing arguments");
        for arg in &self.args.args {
            self.stack.push(Data::from_str(arg));
        }
        info!("Arguments pushed to stack");

        info!("Loading main file: {}", self.args.file);
        let mut main_file = File::new("main".to_string(), self.args.file.clone())?;
        main_file.read()?;
        self.main_file = Some(main_file);

        info!("Main file loaded");
        Ok(())
    }

    pub fn lex(&mut self) -> Result<()> {
        info!("Lexing main file");
        let main_file = self.main_file.as_ref().unwrap();
        let mut tokens = lex(
            main_file.contents.as_str(),
            self.token_types.clone(),
            main_file.path.clone(),
        )?;
        self.tokens.append(&mut tokens);

        info!("Main file lexed");
        Ok(())
    }

    pub fn after_lex(&mut self) -> Result<()> {
        info!("Starting after-lexing");

        let tokens = self.tokens.clone();
        for token in &mut self.tokens {
            let token_type = token.get_type(self.token_types.clone())?.type_;
            if token_type == TokenKind::Statement {
                // Go through all the tokens and find the matching end token
                let mut end_token = None;
                let mut depth = 0;
                for (i, t) in tokens.iter().enumerate() {
                    let token_type = t.get_type(self.token_types.clone())?;
                    if token_type.type_ == TokenKind::Statement && token_type.name == "end" {
                        if t.data == token.data {
                            if depth == 0 {
                                end_token = Some(i);
                                debug!("Found end token for token at {}:{}", token.line, token.col);
                                break;
                            } else {
                                depth -= 1;
                            }
                        } else if token_type.type_ == TokenKind::Statement {
                            depth += 1;
                        }
                    }
                }
                token.data = Data::from_int(end_token.unwrap() as i32);
            }
        }

        info!("After-lexing complete");
        Ok(())
    }

    pub fn interpret(&mut self) -> Result<()> {
        info!("Interpreting tokens");

        while self.pc < self.tokens.len() {
            self.interpret_step()?;
            self.pc += 1;
        }

        info!("Interpretation complete");
        Ok(())
    }

    fn interpret_step(&mut self) -> Result<()> {
        if self.pc >= self.tokens.len() {
            return Err(anyhow::anyhow!("Program counter out of bounds"));
        }
        let token = &self.tokens[self.pc];

        debug!("Interpreting token: {:?}", token);
        match token.exec(&self.token_types, &mut self.stack, &mut self.marks, &mut self.pc) {
            Ok(_) => {}
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Error interpreting token at {}:{}:{}: {}:\n{}",
                    token.file.to_string().blue().bold(),
                    token.line.to_string().bright_black().bold(),
                    token.col.to_string().bright_black().bold(),
                    e.to_string().red().bold(),
                    token.vis
                ));
            }
        };

        Ok(())
    }
}
