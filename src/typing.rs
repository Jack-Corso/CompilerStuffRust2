use std::rc::Rc;
#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Int32,
    FuncType {
        return_type: Box<Type>,
        param_types: Vec<Box<Type>>,
    },
    Unknown,
}

impl Type {
    pub fn get_size(&self) -> usize {
        match &self {
            Type::Int32 => 4,
            Type::FuncType { return_type, .. } => return_type.get_size(),
            Type::Unknown => 0,
        }
    }
    
}
