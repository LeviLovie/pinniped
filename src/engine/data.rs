use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    None,
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Data::String(s) => write!(f, "{}", s),
            Data::Int(i) => write!(f, "{}", i),
            Data::Float(fl) => write!(f, "{}", fl),
            Data::Bool(b) => write!(f, "{}", b),
            Data::None => write!(f, "None"),
        }
    }
}

impl Data {
    pub fn from_any(s: &str) -> Self {
        if let Ok(i) = s.parse::<i32>() {
            return Data::Int(i);
        }

        if let Ok(f) = s.parse::<f32>() {
            return Data::Float(f);
        }

        if s == "true" || s == "false" {
            return Data::Bool(s.parse::<bool>().unwrap());
        }

        if s == "None" || s == "none" || s == "" {
            return Data::None;
        }

        Data::String(s.to_string())
    }

    pub fn from_int(i: i32) -> Self {
        Data::Int(i)
    }

    pub fn from_float(f: f32) -> Self {
        Data::Float(f)
    }

    pub fn from_bool(b: bool) -> Self {
        Data::Bool(b)
    }

    pub fn from_string(s: String) -> Self {
        Data::String(s)
    }

    pub fn from_str(s: &str) -> Self {
        Data::String(s.to_string())
    }

    pub fn is_false(&self) -> bool {
        match &self {
            Data::Bool(b) => !*b,
            Data::Int(i) => *i == 0,
            Data::Float(f) => *f == 0.0,
            Data::String(s) => s.is_empty(),
            Data::None => true,
        }
    }

    pub fn is_true(&self) -> bool {
        !self.is_false()
    }

    pub fn is_int(&self) -> bool {
        match &self {
            Data::Int(_) => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match &self {
            Data::Float(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match &self {
            Data::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match &self {
            Data::String(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match &self {
            Data::Int(_) | Data::Float(_) => true,
            _ => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match &self {
            Data::None => true,
            _ => false,
        }
    }

    pub fn as_int(&self) -> Result<i32> {
        match &self {
            Data::Int(i) => Ok(*i),
            _ => Err(anyhow::anyhow!("Data is not an int")),
        }
    }

    pub fn as_float(&self) -> Result<f32> {
        match &self {
            Data::Float(f) => Ok(*f),
            _ => Err(anyhow::anyhow!("Data is not a float")),
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        match &self {
            Data::Bool(b) => Ok(*b),
            _ => Err(anyhow::anyhow!("Data is not a bool")),
        }
    }

    pub fn as_string(&self) -> Result<String> {
        match &self {
            Data::String(s) => Ok(s.to_string()),
            _ => Err(anyhow::anyhow!("Data is not a string")),
        }
    }

    pub fn as_str(&self) -> Result<&str> {
        match &self {
            Data::String(s) => Ok(s),
            _ => Err(anyhow::anyhow!("Data is not a string")),
        }
    }

    pub fn type_name(&self) -> &str {
        match &self {
            Data::Int(_) => "int",
            Data::Float(_) => "float",
            Data::Bool(_) => "bool",
            Data::String(_) => "string",
            Data::None => "None",
        }
    }

    pub fn check_type(&self, other: &Data) -> bool {
        match &self {
            Data::Int(_) => other.is_int(),
            Data::Float(_) => other.is_float(),
            Data::Bool(_) => other.is_bool(),
            Data::String(_) => other.is_string(),
            Data::None => other.is_none(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn from_int() {
        let data = Data::from_int(42);
        assert_eq!(data, Data::Int(42));
        assert!(data.is_int());
    }

    #[test]
    fn from_float() {
        let data = Data::from_float(42.0);
        assert_eq!(data, Data::Float(42.0));
        assert!(data.is_float());
    }

    #[test]
    fn from_bool() {
        let data = Data::from_bool(true);
        assert_eq!(data, Data::Bool(true));
        assert!(data.is_bool());
    }

    #[test]
    fn from_string() {
        let data = Data::from_string("42".to_string());
        assert_eq!(data, Data::String("42".to_string()));
        assert!(data.is_string());
    }

    #[test]
    fn from_str() {
        let data = Data::from_str("42");
        assert_eq!(data, Data::String("42".to_string()));
        assert!(data.is_string());
    }

    #[test]
    fn from_any() {
        let data = Data::from_any("42");
        assert_eq!(data, Data::Int(42));
        assert!(data.is_int());

        let data = Data::from_any("42.0");
        assert_eq!(data, Data::Float(42.0));
        assert!(data.is_float());

        let data = Data::from_any("true");
        assert_eq!(data, Data::Bool(true));
        assert!(data.is_bool());

        let data = Data::from_any("false");
        assert_eq!(data, Data::Bool(false));
        assert!(data.is_bool());

        let data = Data::from_any("None");
        assert_eq!(data, Data::None);
        assert!(data.is_none());

        let data = Data::from_any("Any");
        assert_eq!(data, Data::String("Any".to_string()));
        assert!(data.is_string());
    }

    #[test]
    fn is_number() {
        let data = Data::Int(42);
        assert!(data.is_number());

        let data = Data::Float(42.0);
        assert!(data.is_number());

        let data = Data::Bool(true);
        assert!(!data.is_number());

        let data = Data::String("42".to_string());
        assert!(!data.is_number());
    }

    #[test]
    fn as_int() {
        let data = Data::Int(42);
        assert_eq!(data.as_int().unwrap(), 42);

        let data = Data::Float(42.0);
        assert!(data.as_int().is_err());

        let data = Data::Bool(true);
        assert!(data.as_int().is_err());

        let data = Data::String("42".to_string());
        assert!(data.as_int().is_err());
    }

    #[test]
    fn as_float() {
        let data = Data::Float(42.0);
        assert_eq!(data.as_float().unwrap(), 42.0);

        let data = Data::Int(42);
        assert!(data.as_float().is_err());

        let data = Data::Bool(true);
        assert!(data.as_float().is_err());

        let data = Data::String("42".to_string());
        assert!(data.as_float().is_err());
    }

    #[test]
    fn as_bool() {
        let data = Data::Bool(true);
        assert_eq!(data.as_bool().unwrap(), true);

        let data = Data::Int(42);
        assert!(data.as_bool().is_err());

        let data = Data::Float(42.0);
        assert!(data.as_bool().is_err());

        let data = Data::String("true".to_string());
        assert!(data.as_bool().is_err());
    }

    #[test]
    fn as_string() {
        let data = Data::String("42".to_string());
        assert_eq!(data.as_string().unwrap(), "42");

        let data = Data::Int(42);
        assert!(data.as_string().is_err());

        let data = Data::Float(42.0);
        assert!(data.as_string().is_err());

        let data = Data::Bool(true);
        assert!(data.as_string().is_err());
    }

    #[test]
    fn as_str() {
        let data = Data::String("42".to_string());
        assert_eq!(data.as_str().unwrap(), "42");

        let data = Data::Int(42);
        assert!(data.as_str().is_err());

        let data = Data::Float(42.0);
        assert!(data.as_str().is_err());

        let data = Data::Bool(true);
        assert!(data.as_str().is_err());
    }

    #[test]
    fn check_type() {
        let data = Data::Int(42);
        assert!(data.check_type(&Data::Int(42)));
        assert!(!data.check_type(&Data::Float(42.0)));
        assert!(!data.check_type(&Data::Bool(true)));
        assert!(!data.check_type(&Data::String("42".to_string())));
        assert!(!data.check_type(&Data::None));

        let data = Data::Float(42.0);
        assert!(!data.check_type(&Data::Int(42)));
        assert!(data.check_type(&Data::Float(42.0)));
        assert!(!data.check_type(&Data::Bool(true)));
        assert!(!data.check_type(&Data::String("42".to_string())));
        assert!(!data.check_type(&Data::None));

        let data = Data::Bool(true);
        assert!(!data.check_type(&Data::Int(42)));
        assert!(!data.check_type(&Data::Float(42.0)));
        assert!(data.check_type(&Data::Bool(true)));
        assert!(!data.check_type(&Data::String("42".to_string())));
        assert!(!data.check_type(&Data::None));

        let data = Data::String("42".to_string());
        assert!(!data.check_type(&Data::Int(42)));
        assert!(!data.check_type(&Data::Float(42.0)));
        assert!(!data.check_type(&Data::Bool(true)));
        assert!(data.check_type(&Data::String("42".to_string())));
        assert!(!data.check_type(&Data::None));

        let data = Data::None;
        assert!(!data.check_type(&Data::Int(42)));
        assert!(!data.check_type(&Data::Float(42.0)));
        assert!(!data.check_type(&Data::Bool(true)));
        assert!(!data.check_type(&Data::String("42".to_string())));
        assert!(data.check_type(&Data::None));
    }

    #[test]
    fn is_false() {
        let data = Data::Int(0);
        assert!(data.is_false());
        let data = Data::Int(42);
        assert!(!data.is_false());

        let data = Data::Float(0.0);
        assert!(data.is_false());
        let data = Data::Float(42.0);
        assert!(!data.is_false());

        let data = Data::Bool(false);
        assert!(data.is_false());
        let data = Data::Bool(true);
        assert!(!data.is_false());

        let data = Data::String("".to_string());
        assert!(data.is_false());
        let data = Data::String("42".to_string());
        assert!(!data.is_false());

        let data = Data::None;
        assert!(data.is_false());
    }

    #[test]
    fn is_true() {
        let data = Data::Int(0);
        assert!(!data.is_true());
        let data = Data::Int(42);
        assert!(data.is_true());

        let data = Data::Float(0.0);
        assert!(!data.is_true());
        let data = Data::Float(42.0);
        assert!(data.is_true());

        let data = Data::Bool(false);
        assert!(!data.is_true());
        let data = Data::Bool(true);
        assert!(data.is_true());

        let data = Data::String("".to_string());
        assert!(!data.is_true());
        let data = Data::String("42".to_string());
        assert!(data.is_true());

        let data = Data::None;
        assert!(!data.is_true());
    }
}
