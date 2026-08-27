use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::Hash;

pub struct StackFrame {
    free_mem: HashMap<usize, Vec<StackAddr>>,
    size: usize,
}

impl StackFrame {
    pub fn new() -> StackFrame {
        StackFrame {
            free_mem: HashMap::new(),
            size: 0,
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

    pub fn size(&self) -> usize {
        self.size
    }

}

#[derive(Clone, Debug)]
pub struct StackAddr {
    offset: usize,
    size: usize,
    addr_str: String,
}

impl Display for StackAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.addr_str)
    }
}

impl Hash for StackAddr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.offset.hash(state);
    }
}

impl StackAddr {
    fn new(offset: usize, size: usize) -> StackAddr {
        StackAddr {
            offset,
            size,
            addr_str: format!("-{offset}(%rbp)"),
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn size(&self) -> usize {
        self.size
    }

}