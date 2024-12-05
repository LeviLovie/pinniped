use anyhow::Result;

use super::engine::data::Data;
use super::engine::lexer::token::{TokenKind, TokenType};
use super::engine::stack::Stack;

pub fn tokens() -> Vec<TokenType> {
    vec![
        // Push a value to the stack
        TokenType::reg(
            TokenKind::Push,
            "push",
            "\\((.+)\\)", // Captures anything exept whitespace inside ()
            |stack: &mut Stack, _, add_value| -> Result<()> {
                stack.push(add_value);
                Ok(())
            },
        ),
        // Pop and print the top value from the stack
        TokenType::reg(
            TokenKind::Function,
            "pop",
            "\\.",
            |stack: &mut Stack, _, _| -> Result<()> {
                print!("{}", stack.pop()?);
                Ok(())
            },
        ),
        // Print \n
        TokenType::reg(TokenKind::Function, "nl", "nl", |_, _, _| -> Result<()> {
            println!();
            Ok(())
        }),
        // Add the top two values from the stack
        TokenType::reg(
            TokenKind::Function,
            "+",
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
            "*",
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
            "^",
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
            "swp",
            |stack: &mut Stack, _, _| -> Result<()> {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(a);
                stack.push(b);
                Ok(())
            },
        ),
        // Rotate the top three values from the stack to the left
        TokenType::reg(
            TokenKind::Function,
            "ror",
            "ror",
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
            "rol",
            "rol",
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
            "clr",
            |stack: &mut Stack, _, _| -> Result<()> {
                stack.clear();
                Ok(())
            },
        ),
        // Debug the stack
        TokenType::reg(
            TokenKind::Function,
            "`",
            "`",
            |stack: &mut Stack, _, _| -> Result<()> {
                println!("Stack debug:\n{}", stack);
                Ok(())
            },
        ),
        // If removes one element from the stack, if that is 0, it skips to the end token
        TokenType::reg(
            TokenKind::Statement,
            "if",
            "if",
            |stack: &mut Stack, pc_counter: &mut usize, data: Data| -> Result<()> {
                let last_element = stack.pop()?;
                if last_element.is_false() {
                    if !data.is_number() {
                        stack.push(last_element);
                        return Err(anyhow::anyhow!("If statement requires a number as the offset. Were tokens linked?"));
                    }
                    *pc_counter += data.as_int()? as usize - 1;
                }
                Ok(())
            },
        ),
        // End token for if
        TokenType::reg(
            TokenKind::Statement,
            "end",
            "end",
            |_, _, _| -> Result<()> {
                Ok(())
            },
        ),
        // Push current pc to the stack
        TokenType::reg(
            TokenKind::Function,
            "here",
            "here",
            |stack: &mut Stack, pc_counter: &mut usize, _| -> Result<()> {
                stack.push(Data::from_int(*pc_counter as i32));
                Ok(())
            },
        ),
        // Jump to the top value of the stack
        TokenType::reg(
            TokenKind::Function,
            "jmp",
            "jmp",
            |stack: &mut Stack, pc_counter: &mut usize, _| -> Result<()> {
                let a = stack.pop()?;
                if !a.is_number() {
                    stack.push(a);
                    return Err(anyhow::anyhow!("Jump requires a number as the offset"));
                }
                *pc_counter = a.as_int()? as usize;
                Ok(())
            },
        ),
    ]
}
