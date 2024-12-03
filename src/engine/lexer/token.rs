use super::super::{stack::Stack, data::Data};

use anyhow::Result;

type TokenFunc = fn(&mut Stack, &mut usize, Data) -> Result<()>;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Function,
    Push,
    Var,
    If,
    Else,
    Loop,
    End,
}

#[derive(Debug, Clone)]
pub struct TokenType {
    pub type_: TokenKind,
    pub regex: String,
    pub func: TokenFunc,
}

impl TokenType {
    pub fn reg(type_: TokenKind, regex: &str, func: TokenFunc) -> Self {
        Self {
            type_,
            regex: regex.to_string(),
            func,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub type_: usize,
    pub data: Data,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(type_: usize, data: Data, line: usize, col: usize) -> Self {
        Self {
            type_,
            data,
            line,
            col,
        }
    }

    pub fn exec(&self, types: &Vec<TokenType>, stack: &mut Stack, pc: &mut usize) -> Result<()> {
        if self.type_ >= types.len() {
            return Err(anyhow::anyhow!("Token type out of bounds: {}", self.type_));
        }

        (types[self.type_].func)(stack, pc, self.data.clone())
    }
}
