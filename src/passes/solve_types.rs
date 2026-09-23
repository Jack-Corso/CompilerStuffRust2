use std::collections::HashMap;
use std::env::var;
use std::process::id;
use crate::parsing::{BlockItem, Expression, Program, Statement};
use crate::typing::Type;

pub fn solve_types(ast: &mut Program) {
    let mut ident_types = HashMap::new();
    for func in ast.body.iter() {
        // cloning func types kinda hurts but whatever
        ident_types.insert(func.name.clone(), func.func_type.clone());
    }

    for func in ast.body.iter_mut() {
        solve_types_block(&mut func.body, &mut ident_types);
    }
}

fn solve_types_block(items: &mut Vec<BlockItem>, ident_types: &mut HashMap<String, Type>) {
    for item in items {
        match item {
            BlockItem::Statement(statement) => {
                solve_types_statement(statement, ident_types);
            },
            BlockItem::VarDeclaration(declaration) => {
                ident_types.insert(declaration.name.clone(), declaration.var_type.clone());
                if declaration.init_value.is_some() {
                    solve_types_expression(declaration.init_value.as_mut().unwrap(), ident_types);
                }
            }
        }
    }
}
fn solve_types_statement(statement: &mut Statement, ident_types: &mut HashMap<String, Type>) {
    match statement {
        Statement::Expression { expression } => {
            solve_types_expression(expression, ident_types);
        },
        Statement::Yield { value } | Statement::Return { value } => {
            if value.is_some() {
                solve_types_expression(value.as_mut().unwrap(), ident_types);
            }
        },
        Statement::Block { items } => {
            solve_types_block(items, ident_types);
        },
        Statement::If { condition, on_true, on_false } => {
            solve_types_expression()
        }
    }
}
fn solve_types_expression(expression: &mut Expression, ident_types: &mut HashMap<String, Type>) {

    match expression {
        Expression::UnaryOp { target, operator, expr_type } => {
            if !matches!(expr_type, Type::Unknown) {
                return;
            }
            solve_types_expression(target, ident_types);
            *expr_type = target.get_type();
        },
        Expression::BinaryOp { left, operator, right, expr_type } => {
            if !matches!(expr_type, Type::Unknown) {
                return;
            }
            solve_types_expression(left, ident_types);
            solve_types_expression(right, ident_types);
            if left.get_type_ref().is_convertable_to(right.get_type_ref()) {
                left.set_type(left.get_type_ref().try_cast(right.get_type_ref()).unwrap());
                *expr_type = right.get_type();
            } else if right.get_type_ref().is_convertable_to(&left.get_type_ref()) {
                right.set_type(right.get_type_ref().try_cast(left.get_type_ref()).unwrap());
                *expr_type = left.get_type();
            } else {
                panic!("Operator '{operator}' cannot be applied to {} and {}", left.get_type_ref(), right.get_type_ref());
            }
        },
        Expression::Var { expr_type, name } => {
            if !matches!(expr_type, Type::Unknown) {
                return;
            }
            *expr_type = ident_types.get(name).unwrap().clone();
        },
        Expression::VarAssignment { expr_type, name, value } => {
            if !matches!(expr_type, Type::Unknown) {
                return;
            }
            *expr_type = ident_types.get(name).unwrap().clone();
            solve_types_expression(value, ident_types);

        },
        Expression::BlockExpression { items, expr_type } => {
            solve_types_block(items, ident_types);
        },
        Expression::Int32Constant { expr_type, .. } => {
            *expr_type = Type::Int32;
        },
    }
}

