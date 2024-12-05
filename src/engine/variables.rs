use super::data::Data;

pub struct Variable {
    pub name: String,
    pub value: Data,
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
}

impl Variable {
    pub fn new(name: String, value: Data) -> Variable {
        Variable { name, value }
    }
}

pub struct Variables {
    variables: Vec<Variable>,
    local: Vec<Variable>,
}

impl Variables {
    pub fn new() -> Variables {
        Variables {
            variables: Vec::new(),
            local: Vec::new(),
        }
    }

    pub fn add(&mut self, name: String, value: Data, local: bool) {
        if local {
            self.local.push(Variable::new(name, value));
        } else {
            self.variables.push(Variable::new(name, value));
        }
    }

    pub fn get(&self, name: &str, local: bool) -> Option<&Data> {
        if local {
            for variable in &self.local {
                if variable.name == name {
                    return Some(&variable.value);
                }
            }
        } else {
            for variable in &self.variables {
                if variable.name == name {
                    return Some(&variable.value);
                }
            }
        }
        None
    }

    pub fn set(&mut self, name: &str, value: Data, local: bool) {
        if local {
            for variable in &mut self.local {
                if variable.name == name {
                    variable.value = value;
                    return;
                }
            }
        } else {
            for variable in &mut self.variables {
                if variable.name == name {
                    variable.value = value;
                    return;
                }
            }
        }
        self.add(name.to_string(), value, local);
    }

    pub fn remove(&mut self, name: &str, local: bool) {
        if local {
            self.local.retain(|variable| variable.name != name);
        } else {
            self.variables.retain(|variable| variable.name != name);
        }
    }

    pub fn remove_globals(&mut self) {
        self.variables.clear();
    }

    pub fn remove_locals(&mut self) {
        self.local.clear();
    }

    pub fn remove_all(&mut self) {
        self.local.clear();
        self.variables.clear();
    }

    pub fn locals(&self) -> &Vec<Variable> {
        &self.local
    }

    pub fn globals(&self) -> &Vec<Variable> {
        &self.variables
    }
}

impl std::fmt::Display for Variables {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Local variables:")?;
        for variable in self.local.iter().rev() {
            writeln!(f, "  {}", variable)?;
        }
        writeln!(f, "Global variables:")?;
        for variable in self.variables.iter().rev() {
            writeln!(f, "  {}", variable)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod variable {
        use super::*;

        #[test]
        fn new() {
            let variable = Variable::new("name".to_string(), Data::Float(42.0));
            assert_eq!(variable.name, "name");
            assert_eq!(variable.value, Data::Float(42.0));
        }

        #[test]
        fn display() {
            let variable = Variable::new("name".to_string(), Data::Float(42.0));
            assert_eq!(format!("{}", variable), "name = 42");
        }
    }

    mod variables {
        use super::*;

        mod local {
            use super::*;

            #[test]
            fn add() {
                let mut variables = Variables::new();
                variables.add("name".to_string(), Data::Float(42.0), true);
                assert_eq!(variables.local.len(), 1);
                assert_eq!(variables.variables.len(), 0);
            }

            #[test]
            fn get() {
                let mut variables = Variables::new();
                variables.add("name".to_string(), Data::Float(42.0), true);
                assert_eq!(variables.get("name", true), Some(&Data::Float(42.0)));
                assert_eq!(variables.get("name", false), None);
            }

            #[test]
            fn set() {
                let mut variables = Variables::new();
                variables.add("name".to_string(), Data::Float(42.0), true);
                variables.set("name", Data::Float(43.0), true);
                assert_eq!(variables.get("name", true), Some(&Data::Float(43.0)));
            }

            #[test]
            fn remove() {
                let mut variables = Variables::new();
                variables.add("name".to_string(), Data::Float(42.0), true);
                variables.remove("name", true);
                assert_eq!(variables.get("name", true), None);
            }
            #[test]
            fn remove_locals() {
                let mut variables = Variables::new();
                variables.add("name".to_string(), Data::Float(42.0), true);
                variables.add("name2".to_string(), Data::Float(43.0), true);
                variables.remove_locals();
                assert_eq!(variables.get("name", true), None);
                assert_eq!(variables.get("name2", true), None);
            }
        }

        mod global {
            use super::*;

            #[test]
            fn add() {
                let mut variables = Variables::new();
                variables.add("name".to_string(), Data::Float(42.0), false);
                assert_eq!(variables.local.len(), 0);
                assert_eq!(variables.variables.len(), 1);
            }

            #[test]
            fn get() {
                let mut variables = Variables::new();
                variables.add("name".to_string(), Data::Float(42.0), false);
                assert_eq!(variables.get("name", false), Some(&Data::Float(42.0)));
                assert_eq!(variables.get("name", true), None);
            }

            #[test]
            fn set() {
                let mut variables = Variables::new();
                variables.add("name".to_string(), Data::Float(42.0), false);
                variables.set("name", Data::Float(43.0), false);
                assert_eq!(variables.get("name", false), Some(&Data::Float(43.0)));
            }

            #[test]
            fn remove() {
                let mut variables = Variables::new();
                variables.add("name".to_string(), Data::Float(42.0), false);
                variables.remove("name", false);
                assert_eq!(variables.get("name", false), None);
            }

            #[test]
            fn remove_globals() {
                let mut variables = Variables::new();
                variables.add("name".to_string(), Data::Float(42.0), false);
                variables.add("name2".to_string(), Data::Float(43.0), false);
                variables.remove_globals();
                assert_eq!(variables.get("name", false), None);
                assert_eq!(variables.get("name2", false), None);
            }
        }
    }
}
