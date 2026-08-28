use std::env::var;
use crate::parsing::{BlockItem, Expression, Function, Program, Statement, VarDeclaration};

pub trait PrettyPrint {
    fn pretty_print(&self, indent: usize);
    
    fn pretty_println(&self, indent: usize) {
        self.pretty_print(indent);
        println!();
    }
}

impl PrettyPrint for Program {
    fn pretty_print(&self, indent: usize) {
        let indent_str = indent_string(indent);
        println!("{}Program{{", indent_str);
        for func in self.body.iter() {
            func.pretty_println(indent + 1);
        }
        print!("{}}}", indent_str);
    }
}

impl PrettyPrint for Function {
    fn pretty_print(&self, indent: usize) {
        let indent_str = indent_string(indent);
        println!("{}Function[\"{}\"] {{", indent_str, self.name);
        for block_item in self.body.iter() {
            block_item.pretty_println(indent + 1);
        }
        print!("{}}}", indent_str);
    }
}

impl PrettyPrint for BlockItem {
    fn pretty_print(&self, indent: usize) {
        match self {
            BlockItem::Statement(statement) => {
                statement.pretty_print(indent);
            },
            BlockItem::VarDeclaration(var_def) => {
                var_def.pretty_print(indent);
            }
        }
    }
}

impl PrettyPrint for Statement {
    fn pretty_print(&self, indent: usize) {
        let indent_str = indent_string(indent);
        match self {
            Statement::Expression { expression } => {
                expression.pretty_print(indent + 1);
            },
            Statement::Block { items } => {
                println!("{}Block {{", indent_str);
                for item in items.iter() {
                    item.pretty_println(indent+1);
                }
                print!("{}}}", indent_str);
            },
            Statement::Return { value } => {
                println!("{}Return {{", indent_str);
                value.pretty_println(indent + 1);
                print!("{}}}", indent_str);
            }
        }
    }
}

impl PrettyPrint for Expression {
    fn pretty_print(&self, indent: usize) {
        let indent_str = indent_string(indent);
        match self {
            Expression::Var { name } => {
                print!("{}Var[\"{}\"]", indent_str, name);
            },
            Expression::UnaryOp { operator, target } => {
                println!("{}UnaryOp[\"{}\"] {{", indent_str, operator);
                target.pretty_println(indent + 1);
                print!("{}}}", indent_str);
            },
            Expression::VarAssignment { name, value } => {
                println!("{}VarAssign[\"{}\"] {{", indent_str, name);
                value.pretty_println(indent + 1);
                print!("{}}}", indent_str);
            },
            Expression::BinaryOp {
                operator,
                left,
                right
            } => {
                println!("{}BinaryOp[\"{}\"] {{", indent_str, operator);
                println!("\t{}left=>{{", indent_str);
                left.pretty_println(indent + 2);
                println!("\t{t}}}\n\t{t}right=>{{", t=indent_str);
                right.pretty_println(indent + 2);
                print!("\t{t}}}\n{t}}}", t=indent_str);
            },
            Expression::Int32Constant { value } => {
                print!("{}Int32Constant[\"{}\"]", indent_str, value);
            }
        };
    }
}

impl PrettyPrint for VarDeclaration {
    fn pretty_print(&self, indent: usize) {
        let indent_str = indent_string(indent);
        print!("{}VarDef[\"{}\"] {{", indent_str, self.name);
        if let Some(expr) = &self.init_value {
            println!();
            expr.pretty_println(indent + 1);
            print!("{}}}", indent_str);
        } else {
            print!("}}");
        }
    }
}
fn indent_string(indent: usize) -> String {
    "   ".repeat(indent)
}