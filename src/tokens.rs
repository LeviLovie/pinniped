use anyhow::Result;

use super::engine::data::Data;
use super::engine::lexer::token::{TokenKind, TokenType};
use super::engine::mark::MarkList;
use super::engine::stack::Stack;
use super::engine::variables::Variables;

pub fn tokens() -> Vec<TokenType> {
    vec![
        // Imports a file from import(CAPTURE)
        TokenType::reg(
            TokenKind::Import,
            "import",
            "^import\\(\"?(.+)\"?\\)",
            |_, _, _, _, _, _| -> Result<()> {
                Ok(())
            },
        ),
        // Push a value to the stack
        TokenType::reg(
            TokenKind::Push,
            "push",
            "^\\((.+)\\)", // Captures anything exept whitespace inside ()
            |stack: &mut Stack, _, _, _, _, add_value| -> Result<()> {
                stack.push(add_value);
                Ok(())
            },
        ),
        // Store a variable from name on the stack
        TokenType::reg(
            TokenKind::Push,
            "push",
            "^>\\((.+)\\)", // Captures anything exept whitespace inside >{}
            |stack: &mut Stack, _, variables: &mut Variables, _, _, name| -> Result<()> {
                if !name.is_string() {
                    stack.push(name);
                    return Err(anyhow::anyhow!("Variable name must be a string"));
                }
                let value = stack.pop()?;
                variables.add(name.as_string()?, value, true);
                Ok(())
            },
        ),
        // Load a variable from the name on the stack
        TokenType::reg(
            TokenKind::Push,
            "load",
            "^<\\((.+)\\)", // Captures anything exept whitespace inside <{}
            |stack: &mut Stack, _, variables: &mut Variables, _, _, name| -> Result<()> {
                if !name.is_string() {
                    stack.push(name);
                    return Err(anyhow::anyhow!("Variable name must be a string"));
                }
                if let Some(value) = variables.get(&name.as_string()?, true) {
                    stack.push(value.clone());
                } else {
                    stack.push(name);
                    return Err(anyhow::anyhow!("Variable not found"));
                }
                Ok(())
            },
        ),
        // Remove a variable from the name on the stack
        TokenType::reg(
            TokenKind::Push,
            "remove",
            "^\\^\\((.+)\\)", // Captures anything exept whitespace inside ^{}
            |stack: &mut Stack, _, variables: &mut Variables, _, _, name| -> Result<()> {
                if !name.is_string() {
                    stack.push(name);
                    return Err(anyhow::anyhow!("Variable name must be a string"));
                }
                variables.remove(&name.as_string()?, true);
                Ok(())
            },
        ),
        // Store a global variable from name on the stack
        TokenType::reg(
            TokenKind::Push,
            "push",
            "^>>\\((.+)\\)", // Captures anything exept whitespace inside >>{}
            |stack: &mut Stack, _, variables: &mut Variables, _, _, name| -> Result<()> {
                if !name.is_string() {
                    stack.push(name);
                    return Err(anyhow::anyhow!("Variable name must be a string"));
                }
                let value = stack.pop()?;
                variables.add(name.as_string()?, value, false);
                Ok(())
            },
        ),
        // Load a global variable from the name on the stack
        TokenType::reg(
            TokenKind::Push,
            "load",
            "^<<\\((.+)\\)", // Captures anything exept whitespace inside <<{}
            |stack: &mut Stack, _, variables: &mut Variables, _, _, name| -> Result<()> {
                if !name.is_string() {
                    stack.push(name);
                    return Err(anyhow::anyhow!("Variable name must be a string"));
                }
                if let Some(value) = variables.get(&name.as_string()?, false) {
                    stack.push(value.clone());
                } else {
                    stack.push(name);
                    return Err(anyhow::anyhow!("Variable not found"));
                }
                Ok(())
            },
        ),
        // Remove a global variable from the name on the stack
        TokenType::reg(
            TokenKind::Push,
            "remove",
            "^\\^\\^\\((.+)\\)", // Captures anything exept whitespace inside ^^{}
            |stack: &mut Stack, _, variables: &mut Variables, _, _, name| -> Result<()> {
                if !name.is_string() {
                    stack.push(name);
                    return Err(anyhow::anyhow!("Variable name must be a string"));
                }
                variables.remove(&name.as_string()?, false);
                Ok(())
            },
        ),
        // Pop and print the top value from the stack
        TokenType::reg(
            TokenKind::Function,
            ".",
            "^\\.$",
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
                print!("{}", stack.pop()?);
                Ok(())
            },
        ),
        // Print the top value from the stack
        TokenType::reg(
            TokenKind::Function,
            ",",
            ",",
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
                let a = match stack.last() {
                    Some(a) => a,
                    None => {
                        return Err(anyhow::anyhow!("Cannot print empty stack"));
                    }
                };
                print!("{}", a);
                Ok(())
            },
        ),
        // Print \n
        TokenType::reg(
            TokenKind::Function,
            "nl",
            "nl",
            |_, _, _, _, _, _| -> Result<()> {
                println!();
                Ok(())
            },
        ),
        // Add the top two values from the stack
        TokenType::reg(
            TokenKind::Function,
            "+",
            "\\+",
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
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
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
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
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
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
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
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
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
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
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
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
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
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
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
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
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
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
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
                let a = stack.pop()?;
                let b = stack.pop()?;
                let c = stack.pop()?;
                stack.push(a);
                stack.push(c);
                stack.push(b);
                Ok(())
            },
        ),
        // Rotate the top three values from the stack to the right
        TokenType::reg(
            TokenKind::Function,
            "rol",
            "rol",
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
                let a = stack.pop()?;
                let b = stack.pop()?;
                let c = stack.pop()?;
                stack.push(b);
                stack.push(a);
                stack.push(c);
                Ok(())
            },
        ),
        // Clear the stack
        TokenType::reg(
            TokenKind::Function,
            "clr",
            "clr",
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
                stack.clear();
                Ok(())
            },
        ),
        // Debug the stack
        TokenType::reg(
            TokenKind::Function,
            "`",
            "`",
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
                println!("Stack debug:\n{}", stack);
                Ok(())
            },
        ),
        // Exit the program with code from the stack
        TokenType::reg(
            TokenKind::Function,
            "exit",
            "exit",
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
                let a = stack.pop()?;
                if a.is_int() {
                    std::process::exit(a.as_int()? as i32);
                } else {
                    stack.push(a);
                    return Err(anyhow::anyhow!("Exit requires an integer value"));
                }
            },
        ),
        // Exit the program with 0 code
        TokenType::reg(
            TokenKind::Function,
            "quit",
            "quit",
            |_, _, _, _, _, _| -> Result<()> {
                std::process::exit(0);
            },
        ),
        // Pushes true if last two elements are equal
        TokenType::reg(
            TokenKind::Function,
            "=",
            "^=",
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
                let a = stack.pop()?;
                let b = stack.pop()?;
                if a.is_number() && b.is_number() {
                    if a.is_int() && b.is_int() {
                        stack.push(Data::from_bool(b.as_int()? == a.as_int()?));
                    } else {
                        stack.push(Data::from_bool(b.as_float()? == a.as_float()?));
                    }
                } else if a.type_name() == b.type_name() {
                    stack.push(Data::from_bool(b == a));
                } else {
                    stack.push(b);
                    stack.push(a);
                    return Err(anyhow::anyhow!("Cannot compare different types"));
                }
                Ok(())
            },
        ),
        // Inverts the last element
        TokenType::reg(
            TokenKind::Function,
            "!",
            "!",
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
                let a = stack.pop()?;
                stack.push(Data::from_bool(a.is_false()));
                Ok(())
            },
        ),
        // Pushes true if last element is less than the second to last element
        TokenType::reg(
            TokenKind::Function,
            "<",
            "<",
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
                let b = stack.pop()?;
                let a = stack.pop()?;
                if a.is_number() && b.is_number() {
                    if a.is_int() && b.is_int() {
                        stack.push(Data::from_bool(b.as_int()? < a.as_int()?));
                    } else {
                        stack.push(Data::from_bool(b.as_float()? < a.as_float()?));
                    }
                } else {
                    stack.push(b);
                    stack.push(a);
                    return Err(anyhow::anyhow!("Cannot compare non-number values"));
                }
                Ok(())
            },
        ),
        // Pushes true if last element is greater than the second to last element
        TokenType::reg(
            TokenKind::Function,
            ">",
            ">",
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
                let b = stack.pop()?;
                let a = stack.pop()?;
                if a.is_number() && b.is_number() {
                    if a.is_int() && b.is_int() {
                        stack.push(Data::from_bool(b.as_int()? > a.as_int()?));
                    } else {
                        stack.push(Data::from_bool(b.as_float()? > a.as_float()?));
                    }
                } else {
                    stack.push(b);
                    stack.push(a);
                    return Err(anyhow::anyhow!("Cannot compare non-number values"));
                }
                Ok(())
            },
        ),
        // Push current pc to the stack
        TokenType::reg(
            TokenKind::Function,
            "here",
            "here",
            |stack: &mut Stack, _, _, _, pc: &mut usize, _| -> Result<()> {
                stack.push(Data::from_int(*pc as i32));
                Ok(())
            },
        ),
        // Jump to a mark
        TokenType::reg(
            TokenKind::Function,
            "jmp",
            "jmp",
            |stack: &mut Stack, _, _, marks: &mut MarkList, pc: &mut usize, _| -> Result<()> {
                let location = stack.pop()?;
                if let Some(new_pc) = marks.get_pc(&location.as_string()?) {
                    *pc = new_pc;
                } else {
                    stack.push(Data::String(location.as_string()?.to_string()));
                    return Err(anyhow::anyhow!("Proc not found"));
                }
                Ok(())
            },
        ),
        // Returns the number of elements in the stack
        TokenType::reg(
            TokenKind::Function,
            "len",
            "len",
            |stack: &mut Stack, _, _, _, _, _| -> Result<()> {
                stack.push(Data::from_int(stack.len() as i32));
                Ok(())
            },
        ),
        // If removes one element from the stack, if that is 0, it skips to the end token
        TokenType::reg(
            TokenKind::If,
            "if",
            "^if",
            |stack: &mut Stack, _, _, _, pc: &mut usize, data: Data| -> Result<()> {
                let last_element = stack.pop()?;
                if last_element.is_false() {
                    if !data.is_number() {
                        stack.push(last_element);
                        return Err(anyhow::anyhow!(
                            "If statement requires a number as the offset. Were tokens linked?"
                        ));
                    }
                    *pc += data.as_int()? as usize - 1;
                }
                Ok(())
            },
        ),
        // End token for if
        TokenType::reg(
            TokenKind::EndIf,
            "endif",
            "endif",
            |_, _, _, _, _, _| -> Result<()> { Ok(()) },
        ),
        // While token
        TokenType::reg(
            TokenKind::While,
            "while",
            "while",
            |_, _, _, _, _, _| -> Result<()> { Ok(()) },
        ),
        // Works just as if
        TokenType::reg(
            TokenKind::Do,
            "do",
            "do",
            |stack: &mut Stack, _, _, _, pc: &mut usize, data: Data| -> Result<()> {
                let last_element = stack.pop()?;
                if last_element.is_false() {
                    if !data.is_number() {
                        stack.push(last_element);
                        return Err(anyhow::anyhow!(
                            "If statement requires a number as the offset. Were tokens linked?"
                        ));
                    }
                    *pc += data.as_int()? as usize + 1;
                }
                Ok(())
            },
        ),
        // End for while returns to the while token in data
        TokenType::reg(
            TokenKind::End,
            "end",
            "end",
            |_, _, _, _, pc: &mut usize, data: Data| -> Result<()> {
                if !data.is_number() {
                    return Err(anyhow::anyhow!(
                        "Endwhile statement requires a number as the offset. Were tokens linked?"
                    ));
                }
                *pc -= data.as_int()? as usize + 1;
                Ok(())
            },
        ),
        // Register a value from the stack to a mark
        TokenType::reg(
            TokenKind::Proc,
            "proc",
            "proc",
            |stack: &mut Stack,
             _,
             _,
             marks: &mut MarkList,
             pc: &mut usize,
             data: Data|
             -> Result<()> {
                let location = stack.pop()?;
                marks.push(location.as_string()?.to_string(), *pc);
                if !data.is_number() {
                    stack.push(location);
                    return Err(anyhow::anyhow!(
                        "Proc statement requires a number as the offset. Were tokens linked?"
                    ));
                }
                *pc += data.as_int()? as usize + 1;
                Ok(())
            },
        ),
        // Calling a procedure (same as a push but with curlies)
        TokenType::reg(
            TokenKind::Function,
            "call",
            "\\{(.+)\\}",
            |_,
             return_stack: &mut Stack,
             _,
             marks: &mut MarkList,
             pc: &mut usize,
             data: Data|
             -> Result<()> {
                return_stack.push(Data::from_int(*pc as i32));
                let location = match marks.get_pc(&data.as_string()?) {
                    Some(a) => a,
                    None => {
                        return Err(anyhow::anyhow!("Proc not found: {}", data));
                    }
                };
                *pc = location as usize;
                Ok(())
            },
        ),
        // Return from a procedure
        TokenType::reg(
            TokenKind::ProcRet,
            "ret",
            "ret",
            |_, return_stack: &mut Stack, _, _, pc: &mut usize, _| -> Result<()> {
                let location = return_stack.pop()?;
                if location.is_int() {
                    *pc = location.as_int()? as usize;
                } else {
                    return Err(anyhow::anyhow!("Return location is not an integer"));
                }
                Ok(())
            },
        ),
    ]
}
