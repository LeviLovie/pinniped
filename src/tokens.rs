use anyhow::Result;

use super::engine::data::Data;
use super::engine::lexer::token::{TokenKind, TokenType};
use super::engine::stack::Stack;

pub fn tokens() -> Vec<TokenType> {
    vec![
        // Push a value to the stack
        TokenType::reg(
            TokenKind::Push,
            "\\((.+)\\)", // Captures anything exept whitespace inside ()
            |stack: &mut Stack, _, add_value| -> Result<()> {
                stack.push(add_value);
                Ok(())
            },
        ),
        // Pop and print the top value from the stack
        TokenType::reg(
            TokenKind::Function,
            "\\.",
            |stack: &mut Stack, _, _| -> Result<()> {
                print!("{}", stack.pop()?);
                Ok(())
            },
        ),
        // Print \n
        TokenType::reg(TokenKind::Function, "nl", |_, _, _| -> Result<()> {
            println!();
            Ok(())
        }),
        // Add the top two values from the stack
        TokenType::reg(
            TokenKind::Function,
            "\\+",
            |stack: &mut Stack, _, _| -> Result<()> {
                let a = stack.pop()?;
                let b = stack.pop()?;
                if a.is_number() && b.is_number() {
                    if a.is_int() && b.is_int() {
                        stack.push(Data::from_int(b.as_int()? + a.as_int()?));
                    } else {
                        stack.push(Data::from_float(b.as_float()? + a.as_float()?));
                    }
                } else {
                    stack.push(b);
                    stack.push(a);
                    return Err(anyhow::anyhow!("Cannot add non-number values"));
                }
                Ok(())
            },
        ),
        // Subtract the top two values from the stack
        TokenType::reg(
            TokenKind::Function,
            "-",
            |stack: &mut Stack, _, _| -> Result<()> {
                let a = stack.pop()?;
                let b = stack.pop()?;
                if a.is_number() && b.is_number() {
                    if a.is_int() && b.is_int() {
                        stack.push(Data::from_int(b.as_int()? - a.as_int()?));
                    } else {
                        stack.push(Data::from_float(b.as_float()? - a.as_float()?));
                    }
                } else {
                    stack.push(b);
                    stack.push(a);
                    return Err(anyhow::anyhow!("Cannot subtract non-number values"));
                }
                Ok(())
            },
        ),
        // Multiply the top two values from the stack
        TokenType::reg(
            TokenKind::Function,
            "\\*",
            |stack: &mut Stack, _, _| -> Result<()> {
                let a = stack.pop()?;
                let b = stack.pop()?;
                if a.is_number() && b.is_number() {
                    if a.is_int() && b.is_int() {
                        stack.push(Data::from_int(b.as_int()? * a.as_int()?));
                    } else {
                        stack.push(Data::from_float(b.as_float()? * a.as_float()?));
                    }
                } else {
                    stack.push(b);
                    stack.push(a);
                    return Err(anyhow::anyhow!("Cannot multiply non-number values"));
                }
                Ok(())
            },
        ),
        // Divide the top two values from the stack
        TokenType::reg(
            TokenKind::Function,
            "/",
            |stack: &mut Stack, _, _| -> Result<()> {
                let a = stack.pop()?;
                let b = stack.pop()?;
                if a.is_number() && b.is_number() {
                    if a.is_int() && b.is_int() {
                        stack.push(Data::from_int(b.as_int()? / a.as_int()?));
                    } else {
                        stack.push(Data::from_float(b.as_float()? / a.as_float()?));
                    }
                } else {
                    stack.push(b);
                    stack.push(a);
                    return Err(anyhow::anyhow!("Cannot divide non-number values"));
                }
                Ok(())
            },
        ),
        // Modulo the top two values from the stack
        TokenType::reg(
            TokenKind::Function,
            "%",
            |stack: &mut Stack, _, _| -> Result<()> {
                let a = stack.pop()?;
                let b = stack.pop()?;
                if a.is_number() && b.is_number() {
                    if a.is_int() && b.is_int() {
                        stack.push(Data::from_int(b.as_int()? % a.as_int()?));
                    } else {
                        stack.push(Data::from_float(b.as_float()? % a.as_float()?));
                    }
                } else {
                    stack.push(b);
                    stack.push(a);
                    return Err(anyhow::anyhow!("Cannot modulo non-number values"));
                }
                Ok(())
            },
        ),
        // Exponentiate the top two values from the stack
        TokenType::reg(
            TokenKind::Function,
            "\\^",
            |stack: &mut Stack, _, _| -> Result<()> {
                let a = stack.pop()?;
                let b = stack.pop()?;
                if a.is_number() && b.is_number() {
                    if a.is_int() && b.is_int() {
                        stack.push(Data::from_int(b.as_int()?.pow(a.as_int()? as u32)));
                    } else {
                        stack.push(Data::from_float(b.as_float()?.powf(a.as_float()?)));
                    }
                } else {
                    stack.push(b);
                    stack.push(a);
                    return Err(anyhow::anyhow!("Cannot exponentiate non-number values"));
                }
                Ok(())
            },
        ),
        // Negate the top value from the stack
        TokenType::reg(
            TokenKind::Function,
            "~",
            |stack: &mut Stack, _, _| -> Result<()> {
                let a = stack.pop()?;
                if a.is_number() {
                    if a.is_int() {
                        stack.push(Data::from_int(-a.as_int()?));
                    } else {
                        stack.push(Data::from_float(-a.as_float()?));
                    }
                } else {
                    stack.push(a);
                    return Err(anyhow::anyhow!("Cannot negate non-number value"));
                }
                Ok(())
            },
        ),
        // Duplicate the top value from the stack
        TokenType::reg(
            TokenKind::Function,
            ":",
            |stack: &mut Stack, _, _| -> Result<()> {
                let a = stack.pop()?;
                stack.push(a.clone());
                stack.push(a);
                Ok(())
            },
        ),
        // Swap the top two values from the stack
        TokenType::reg(
            TokenKind::Function,
            "swp",
            |stack: &mut Stack, _, _| -> Result<()> {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(a);
                stack.push(b);
                Ok(())
            },
        ),
        // Swap the top three values from the stack
        TokenType::reg(
            TokenKind::Function,
            "rot",
            |stack: &mut Stack, _, _| -> Result<()> {
                let a = stack.pop()?;
                let b = stack.pop()?;
                let c = stack.pop()?;
                stack.push(c);
                stack.push(b);
                stack.push(a);
                Ok(())
            },
        ),
        // Rotate the top three values from the stack to the left
        TokenType::reg(
            TokenKind::Function,
            "rol",
            |stack: &mut Stack, _, _| -> Result<()> {
                let a = stack.pop()?;
                let b = stack.pop()?;
                let c = stack.pop()?;
                stack.push(b);
                stack.push(a);
                stack.push(c);
                Ok(())
            },
        ),
        // Rotate the top three values from the stack to the right
        TokenType::reg(
            TokenKind::Function,
            "ror",
            |stack: &mut Stack, _, _| -> Result<()> {
                let a = stack.pop()?;
                let b = stack.pop()?;
                let c = stack.pop()?;
                stack.push(a);
                stack.push(c);
                stack.push(b);
                Ok(())
            },
        ),
        // Clear the stack
        TokenType::reg(
            TokenKind::Function,
            "clr",
            |stack: &mut Stack, _, _| -> Result<()> {
                stack.clear();
                Ok(())
            },
        ),
        // Duplicate the top value from the stack n times
        TokenType::reg(
            TokenKind::Function,
            "dup\\((\\d+)\\)",
            |stack: &mut Stack, _, add_value| -> Result<()> {
                let n = add_value.as_int()?;
                let a = stack.pop()?;
                for _ in 0..n {
                    stack.push(a.clone());
                }
                stack.push(a);
                Ok(())
            },
        ),
        // Debug the stack
        TokenType::reg(
            TokenKind::Var,
            "`",
            |stack: &mut Stack, _, _| -> Result<()> {
                println!("Stack debug:\n{}", stack);
                Ok(())
            },
        ),
    ]
}
