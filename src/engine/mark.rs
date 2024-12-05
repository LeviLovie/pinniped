#[derive(Debug, Clone, PartialEq)]
pub struct Mark {
    pub name: String,
    pub pc: usize,
}

impl std::fmt::Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.pc)
    }
}

impl Mark {
    pub fn new(name: String, pc: usize) -> Self {
        Self { name, pc }
    }
}

#[derive(Debug, Clone)]
pub struct MarkList {
    marks: Vec<Mark>,
}

impl MarkList {
    pub fn new() -> Self {
        Self { marks: Vec::new() }
    }

    pub fn push(&mut self, mark: Mark) {
        self.marks.push(mark);
    }

    pub fn pop(&mut self) -> Option<Mark> {
        self.marks.pop()
    }

    pub fn exists(&self, name: &str) -> bool {
        self.marks.iter().any(|m| m.name == name)
    }

    pub fn get(&self, name: &str) -> Option<&Mark> {
        self.marks.iter().find(|m| m.name == name)
    }

    pub fn get_pc(&self, name: &str) -> Option<usize> {
        self.get(name).map(|m| m.pc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark() {
        let mark = Mark::new("test".to_string(), 0);
        assert_eq!(mark.to_string(), "test: 0");
    }

    #[test]
    fn test_mark_list() {
        let mut mark_list = MarkList::new();
        mark_list.push(Mark::new("test".to_string(), 0));
        assert_eq!(mark_list.exists("test"), true);
        assert_eq!(mark_list.get_pc("test"), Some(0));
        assert_eq!(mark_list.get_pc("test2"), None);
        assert_eq!(mark_list.pop(), Some(Mark::new("test".to_string(), 0)));
    }
}
