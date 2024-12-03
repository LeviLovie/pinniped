use anyhow::Result;
use log::info;

use super::file::File;
use super::lexer::{lexer::lex, token::{Token, TokenType}};
use super::data::Data;
use super::stack::*;
use crate::args::Args;

pub struct Machine {
    args: Args,
    stack: Stack,
    token_types: Vec<TokenType>,
    main_file: Option<File>,
    tokens: Vec<Token>,
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
            pc: 0,
        }
    }

    pub fn register_tokens(&mut self, tokens: Vec<TokenType>) {
        self.token_types.extend(tokens);
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

        Ok(())
    }

    pub fn lex(&mut self) -> Result<()> {
        info!("Lexing main file");
        let main_file = self.main_file.as_ref().unwrap();
        let mut tokens = lex(main_file.contents.as_str(), self.token_types.clone())?;
        self.tokens.append(&mut tokens);

        Ok(())
    }

    pub fn interpret(&mut self) -> Result<()> {
        info!("Interpreting tokens");

        while self.pc < self.tokens.len() {
            self.interpret_step()?;
            self.pc += 1;
        }

        Ok(())
    }

    fn interpret_step(&mut self) -> Result<()> {
        if self.pc >= self.tokens.len() {
            return Err(anyhow::anyhow!("Program counter out of bounds"));
        }
        let token = &self.tokens[self.pc];

        info!("Interpreting token: {:?}", token);
        token.exec(&self.token_types, &mut self.stack, &mut self.pc)?;

        Ok(())
    }
}
