use super::super::{data::Data, mark::MarkList, stack::Stack};

use anyhow::Result;

type TokenFunc = fn(&mut Stack, &mut MarkList, &mut usize, Data) -> Result<()>;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Function,
    Push,
    If,
    EndIf,
    While,
    Do,
    End,
}

#[derive(Debug, Clone)]
pub struct TokenType {
    pub type_: TokenKind,
    pub name: String,
    pub regex: String,
    pub func: TokenFunc,
}

impl TokenType {
    pub fn reg(type_: TokenKind, name: &str, regex: &str, func: TokenFunc) -> Self {
        Self {
            type_,
            name: name.to_string(),
            regex: regex.to_string(),
            func,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub type_: usize,
    pub data: Data,
    pub file: String,
    pub line: usize,
    pub col: usize,
    pub vis: String,
}

impl Token {
    pub fn new(
        type_: usize,
        data: Data,
        file: String,
        line: usize,
        col: usize,
        vis: String,
    ) -> Self {
        Self {
            type_,
            data,
            file,
            line,
            col,
            vis,
        }
    }

    pub fn get_type(&self, token_types: Vec<TokenType>) -> Result<TokenType> {
        if self.type_ >= token_types.len() {
            return Err(anyhow::anyhow!("Token type out of bounds: {}", self.type_));
        }

        Ok(token_types[self.type_].clone())
    }

    pub fn exec(
        &self,
        types: &Vec<TokenType>,
        stack: &mut Stack,
        marks: &mut MarkList,
        pc: &mut usize,
    ) -> Result<()> {
        if self.type_ >= types.len() {
            return Err(anyhow::anyhow!("Token type out of bounds: {}", self.type_));
        }

        (types[self.type_].func)(stack, marks, pc, self.data.clone())
    }
}
