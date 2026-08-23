use std::rc::Rc;
#[derive(Clone)]
pub enum Type {
    Int32,
    FuncType {
        return_type: Box<Type>,
        param_types: Vec<Box<Type>>,
    },
   
}

