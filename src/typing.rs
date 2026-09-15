use std::fmt::Display;
use std::rc::Rc;
#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Int32,
    FuncType {
        return_type: Box<Type>,
        param_types: Vec<Box<Type>>,
    },
    Unknown,
    Any,
    Casted { from: Box<Type>, to: Box<Type> },
}

impl Type {
    pub fn get_size(&self) -> usize {
        match &self {
            Type::Int32 => 4,
            Type::FuncType { return_type, .. } => return_type.get_size(),
            Type::Unknown => 0,
            Type::Any => 0,
            Type::Casted { from, .. } => from.get_size()
        }
    }
    
    pub fn is_convertable_to(&self, other: &Type) -> bool {
        match &self {
            Type::Int32 => matches!(other, Type::Int32),
            Type::FuncType { return_type, param_types } => {
                panic!("Func Type should not be used in a convertable check.");
            },
            Type::Unknown => {
                panic!("Unknown Type should not be used in a convertable check.");
            },
            Type::Casted { to, .. } => {
                to.is_convertable_to(other)
            }
            Type::Any => true,
        }
    }
    
    pub fn try_cast(&self, other: &Type) -> Option<Type> {
       
        if self.is_convertable_to(other) {
            if self == other {
                return Some(self.clone());
            }
            return Some(Type::Casted { from: Box::new(self.clone()), to: Box::new(other.clone()) });
        }
        return None;
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int32 => write!(f, "i32"),
            Type::FuncType { return_type, param_types } => {
                write!(f, "fn({}) -> {}", param_types.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", "), return_type.to_string())
            },
            Type::Unknown => write!(f, "unknown"),
            Type::Any => write!(f, "any"),
            Type::Casted { from, to } => {
                write!(f, "cast[{from} -> {to}]")
            }
        }
    }
}
