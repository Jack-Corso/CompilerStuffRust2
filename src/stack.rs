use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::rc::{Rc, Weak};
use crate::parsing::{BlockItem, Expression, Function, Statement};

pub struct StackFrame {
    free_mem: HashMap<usize, Vec<StackAddr>>,
    size: usize,
    var_map: HashMap<String, StackAddr>,
}

impl StackFrame {
    fn new() -> StackFrame {
        StackFrame {
            free_mem: HashMap::new(),
            size: 0,
            var_map: HashMap::new(),
        }
    }

    pub fn alloc(&mut self, size: usize) {
        let addr = StackAddr::new(self.size, size);
        self.size += size;
        if !self.free_mem.contains_key(&size) {
            self.free_mem.insert(size, Vec::new());
        }
        self.free_mem.get_mut(&size).unwrap().push(addr);
    }

    pub fn has_free(&self, size: usize) -> bool {
        self.free_mem.contains_key(&size) && !self.free_mem.get(&size).unwrap().is_empty()
    }

    pub fn alloc_if_missing(&mut self, size: usize) {
        if !self.has_free(size) {
            self.alloc(size);
        }
    }

    pub fn reserve(&mut self, size: usize) -> StackAddr {
        let error_msg = "No available stack mem with the given size";
        return self.free_mem.get_mut(&size).expect(error_msg).pop().expect(error_msg);
    }

    pub fn reserve_var(&mut self, name: String, size: usize) -> &StackAddr {
        if self.var_map.contains_key(&name) {
            panic!("Variable {name} already exists in this scope");
        }
        let addr = StackAddr::new(self.size, size);
        self.size += size;
        self.var_map.insert(name.clone(), addr);
        return self.var_map.get(&name).unwrap();
    }

    pub fn get_var(&self, name: &String) -> &StackAddr {
        self.var_map.get(name).expect("Variable does not exist in this scope")
    }

    pub fn free_var(&mut self, name: &String) {
        let addr = self.var_map.remove(name).expect("Tried freeing var that doesn't exist");
        self.free_mem.get_mut(&addr.size).unwrap().push(addr);
    }

    pub fn free(&mut self, addr: StackAddr) {
        if addr.is_view {
            panic!("Cannot free a view of an address");
        }
        let addr_vec = self.free_mem.get_mut(&addr.size).unwrap();
        if (addr_vec.contains(&addr)) {
            panic!("Tried freeing address twice: {addr}")
        }
        addr_vec.push(addr);
    }

    pub fn size(&self) -> usize {
        self.size
    }

}

#[derive(Clone, Copy, Debug, Eq)]
pub struct StackAddr {
    offset: usize,
    size: usize,
    is_view: bool,
}

impl Display for StackAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "-{}(%rbp)", self.offset)
    }
}

impl Hash for StackAddr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.offset.hash(state);
    }
}
impl PartialEq for StackAddr {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
    }
}

impl StackAddr {
    fn new(offset: usize, size: usize) -> StackAddr {
        StackAddr {
            offset,
            size,
            is_view: false,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get_n_byte_view(&self, num_bytes: usize) -> StackAddr {
        if num_bytes > self.size {
            panic!("Cannot create view larger than existing address")
        } 
        if self.is_view {
            panic!("Cannot create a view of ")
        }
        let offset = self.offset + (self.size - num_bytes);
        let view = StackAddr {
            offset,
            size: num_bytes,
            is_view: true,
        };
        return view;
    }
    

}

pub fn create_stack_frame(func: &Function) -> StackFrame {
    let mut stack_frame = StackFrame::new();
    update_stack_frame_block(&mut stack_frame, &func.body);

    stack_frame
}

fn update_stack_frame_block(stack_frame: &mut StackFrame, block_items: &Vec<BlockItem>) {
    let mut scope_vars: Vec<StackAddr> = Vec::new();
    for block_item in block_items {
        match block_item {
            BlockItem::Statement(statement) => {
                update_stack_frame_statement(stack_frame, statement);
            },
            BlockItem::VarDeclaration(var_declaration) => {
                let var_size = var_declaration.var_type.get_size();
                stack_frame.alloc_if_missing(var_size);
                scope_vars.push(stack_frame.reserve(var_size));
            }
        }
    }
    for addr in scope_vars {
        stack_frame.free(addr);
    }
}

fn update_stack_frame_statement(stack_frame: &mut StackFrame, statement: &Statement) {
    match statement {
        Statement::Expression { expression} => {
            update_stack_frame_expression(stack_frame, expression);
        },
        Statement::Block { items } => {
            update_stack_frame_block(stack_frame, items);
        },
        Statement::Return { value } | Statement::Yield { value } => {
            if let Some(expr) = value {
                update_stack_frame_expression(stack_frame, expr);
            }
        },
    }
}

fn update_stack_frame_expression(stack_frame: &mut StackFrame, expression: &Expression) {
    match expression {
        Expression::BinaryOp {
            left,
            right,
            ..
        } => {
            update_stack_frame_expression(stack_frame, right);
            stack_frame.alloc_if_missing(4);
            let temp_addr = stack_frame.reserve(4);
            update_stack_frame_expression(stack_frame, left);
        },
        Expression::VarAssignment { value, .. } => {
            update_stack_frame_expression(stack_frame, value);
        },
        Expression::UnaryOp { target, .. } => {
            update_stack_frame_expression(stack_frame, target);
        },
        Expression::Var { .. } => {},
        Expression::Int32Constant { .. } => {},
        Expression::BlockExpression { items, .. } => {
            update_stack_frame_block(stack_frame, items);
        }
        
    }
}