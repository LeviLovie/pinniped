use anyhow::Result;

use super::data::Data;

#[derive(Debug)]
pub struct Stack {
    stack: Vec<Data>,
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let result = self
            .stack
            .iter()
            .map(|d| format!("{}", d))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}", result)
    }
}

impl Stack {
    pub fn new() -> Stack {
        Stack { stack: Vec::new() }
    }

    pub fn elements(&self) -> &Vec<Data> {
        &self.stack
    }

    pub fn push(&mut self, element: Data) {
        self.stack.push(element);
    }

    pub fn push_int(&mut self, value: i32) {
        self.stack.push(Data::Int(value));
    }

    pub fn push_float(&mut self, value: f32) {
        self.stack.push(Data::Float(value));
    }

    pub fn push_str(&mut self, value: String) {
        self.stack.push(Data::String(value));
    }

    pub fn pop(&mut self) -> Result<Data> {
        if self.stack.is_empty() {
            return Err(anyhow::anyhow!("Cannot pop from an empty stack"));
        }

        Ok(self.stack.pop().unwrap())
    }

    pub fn last(&self) -> Option<&Data> {
        self.stack.last()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_push() {
        let mut stack = super::Stack::new();
        stack.push(super::Data::Int(42));
        stack.push(super::Data::Float(42.0));
        stack.push(super::Data::String("42".to_string()));

        assert_eq!(stack.stack.len(), 3);
        assert_eq!(stack.stack[0], super::Data::Int(42));
        assert_eq!(stack.stack[1], super::Data::Float(42.0));
        assert_eq!(stack.stack[2], super::Data::String("42".to_string()));
    }

    #[test]
    fn test_push_int() {
        let mut stack = super::Stack::new();
        stack.push_int(42);
        stack.push_int(42);

        assert_eq!(stack.stack.len(), 2);
        assert_eq!(stack.stack[0], super::Data::Int(42));
        assert_eq!(stack.stack[1], super::Data::Int(42));
    }

    #[test]
    fn test_push_float() {
        let mut stack = super::Stack::new();
        stack.push_float(42.0);
        stack.push_float(42.0);

        assert_eq!(stack.stack.len(), 2);
        assert_eq!(stack.stack[0], super::Data::Float(42.0));
        assert_eq!(stack.stack[1], super::Data::Float(42.0));
    }

    #[test]
    fn test_push_str() {
        let mut stack = super::Stack::new();
        stack.push_str("42".to_string());
        stack.push_str("42".to_string());

        assert_eq!(stack.stack.len(), 2);
        assert_eq!(stack.stack[0], super::Data::String("42".to_string()));
        assert_eq!(stack.stack[1], super::Data::String("42".to_string()));
    }

    #[test]
    fn test_pop() {
        let mut stack = super::Stack::new();
        stack.push(super::Data::Int(42));
        stack.push(super::Data::Float(42.0));
        stack.push(super::Data::String("42".to_string()));

        assert_eq!(stack.pop().unwrap(), super::Data::String("42".to_string()));
        assert_eq!(stack.pop().unwrap(), super::Data::Float(42.0));
        assert_eq!(stack.pop().unwrap(), super::Data::Int(42));
        assert!(stack.pop().is_err());
    }

    #[test]
    fn test_pop_empty() {
        let mut stack = super::Stack::new();
        assert!(stack.pop().is_err());
    }

    #[test]
    fn test_display() {
        let mut stack = super::Stack::new();
        stack.push(super::Data::Int(42));

        assert_eq!(format!("{}", stack).contains("42"), true);
    }

    #[test]
    fn test_display_empty() {
        let stack = super::Stack::new();
        assert_eq!(format!("{}", stack).is_empty(), true);
    }
}
