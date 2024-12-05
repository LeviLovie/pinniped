use anyhow::Result;
use colored::Colorize;
use log::{debug, info};

use super::data::Data;
use super::file::File;
use super::lexer::{
    lexer::lex,
    token::{Token, TokenType},
};
use super::mark::MarkList;
use super::stack::Stack;
use super::variables::Variables;
use crate::args::Args;

pub struct Machine {
    args: Args,
    stack: Stack,
    return_stack: Stack,
    token_types: Vec<TokenType>,
    main_file: Option<File>,
    tokens: Vec<Token>,
    marks: MarkList,
    variables: Variables,
    pc: usize,
}

impl Machine {
    pub fn new(args: Args) -> Self {
        Self {
            args,
            stack: Stack::new(),
            return_stack: Stack::new(),
            token_types: Vec::new(),
            main_file: None,
            tokens: Vec::new(),
            marks: MarkList::new(),
            variables: Variables::new(),
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
        let mut imported_files = Vec::new();
        let main_file = self.main_file.as_ref().unwrap();
        imported_files.push(main_file.path.clone());
        let mut tokens = lex(
            main_file.contents.as_str(),
            self.token_types.clone(),
            main_file.path.clone(),
            &mut imported_files,
        )?;
        debug!("Imported files: {:?}", imported_files);
        self.tokens.append(&mut tokens);

        info!("Main file lexed");
        Ok(())
    }

    pub fn interpret(&mut self) -> Result<()> {
        info!("Interpreting tokens");

        if self.args.debug_inter {
            println!("Press enter to go to the next token");
        }

        while self.pc < self.tokens.len() {
            if self.args.debug_inter {
                let token = &self.tokens[self.pc];
                let token_type = token.get_type(self.token_types.clone())?;
                let data = token.data.to_string();
                let quote = "\"".bright_black();
                let colon = ":".bright_black();
                let coma = ",".bright_black();
                println!(
                    "\n{}{} {}{}{}",
                    "Line".blue().bold(),
                    colon,
                    quote,
                    token.vis,
                    quote
                );
                if self.stack.len() != 0 {
                    print!("{}{} ", "Stack".blue().bold(), colon);
                    for (i, element) in self.stack.elements().iter().enumerate() {
                        if i % 5 == 0 && i != 0 {
                            print!("       ");
                        }
                        print!("{}{}{}", quote, element.to_string(), quote);
                        if i % 5 == 4 || i == self.stack.len() - 1 {
                            println!();
                        } else {
                            print!(", ");
                        }
                    }
                }
                if self.return_stack.len() != 0 {
                    print!("{}{} ", "Ret stack".blue().bold(), colon);
                    for (i, element) in self.return_stack.elements().iter().enumerate() {
                        if i % 5 == 0 && i != 0 {
                            print!("         ");
                        }
                        print!("{}{}{}", quote, element.to_string(), quote);
                        if i % 5 == 4 || i == self.return_stack.len() - 1 {
                            println!();
                        } else {
                            print!(", ");
                        }
                    }
                }
                let local_variables = self.variables.locals();
                if local_variables.len() != 0 {
                    print!("{}{} ", "Loc vars".blue().bold(), colon);
                    for (i, variable) in local_variables.iter().enumerate() {
                        if i % 5 == 0 && i != 0 {
                            print!("        ");
                        }
                        print!("{}{}{}", quote, variable.to_string(), quote);
                        if i % 5 == 4 || i == local_variables.len() - 1 {
                            println!();
                        } else {
                            print!(", ");
                        }
                    }
                }
                let global_variables = self.variables.globals();
                if global_variables.len() != 0 {
                    print!("{}{} ", "Glo vars".blue().bold(), colon);
                    for (i, variable) in global_variables.iter().enumerate() {
                        if i % 5 == 0 && i != 0 {
                            print!("        ");
                        }
                        print!("{}{}{}", quote, variable.to_string(), quote);
                        if i % 5 == 4 || i == global_variables.len() - 1 {
                            println!();
                        } else {
                            print!(", ");
                        }
                    }
                }
                // println!("PC: {:<5}; Token: \"{}\"; Data: \"{}\"", self.pc, token_type.name, data);
                print!("{}{} {}{} ", "PC".blue().bold(), colon, self.pc, coma);
                print!(
                    "{}{} {}{}{}{} ",
                    "Token".blue().bold(),
                    colon,
                    quote,
                    token_type.name,
                    quote,
                    coma
                );
                println!(
                    "{}{} {}{}{}",
                    "Data".blue().bold(),
                    colon,
                    quote,
                    data,
                    quote
                );
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
            }
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
        match token.exec(
            &self.token_types,
            &mut self.stack,
            &mut self.return_stack,
            &mut self.variables,
            &mut self.marks,
            &mut self.pc,
        ) {
            Ok(_) => {}
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Error interpreting token at {}:{}:{}: {}: \"{}\"",
                    token.file.to_string().blue().bold(),
                    token.line.to_string().bright_black().bold(),
                    token.col.to_string().bright_black().bold(),
                    e.to_string().red().bold(),
                    token.get_type(self.token_types.clone())?.name
                ));
            }
        };

        Ok(())
    }
}
