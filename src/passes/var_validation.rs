use std::collections::HashSet;
use crate::parsing::{BlockItem, Expression, Program, Statement};

pub fn validate_vars(ast: &Program) {
    for func in ast.body.iter() {
        validate_vars_block(&func.body, &mut HashSet::new());
    }
}

fn validate_vars_block(block_items: &Vec<BlockItem>, vars: &mut HashSet<String>) {
    let mut to_remove = HashSet::new();
    for block_item in block_items {
        match block_item {
            BlockItem::VarDeclaration(var_declaration) => {
                if vars.insert(var_declaration.name.clone()) {
                    if !to_remove.insert(var_declaration.name.clone()) {
                        panic!("Duplicate variable defined in scope: {}", var_declaration.name);
                    }
                }
            },
            BlockItem::Statement(statement) => {
                validate_vars_statement(statement, vars);
            }
        }
    }

    vars.retain(|var| !to_remove.contains(var));
}

fn validate_vars_statement(statement: &Statement, vars: &mut HashSet<String>) {
    match statement {
        Statement::Expression { expression } => {
            validate_vars_expression(expression, vars);
        },
        Statement::Block { items } => {
            validate_vars_block(items, vars);
        },
        Statement::Return { value } => {
            if let Some(expr) = value {
                validate_vars_expression(expr, vars);
            }
        },
        Statement::Yield { value } => {
            if let Some(expr) = value {
                validate_vars_expression(expr, vars);
            }
        },
        Statement::If { condition, on_true, on_false } => {
            validate_vars_expression(condition, vars);
            validate_vars_statement(on_true, vars);
            if let Some(statement) = on_false {
                validate_vars_statement(statement, vars);
            }
        }
    }
}

fn validate_vars_expression(expression: &Expression, vars: &mut HashSet<String>) {
    match expression {
        Expression::Var { name, .. } => {
            if !vars.contains(name) {
                panic!("Variable '{}' used before definition", name);
            }
        },
        Expression::VarAssignment { name, value, .. } => {
            if !vars.contains(name) {
                panic!("Variable '{}' used before definition", name);
            }
        },
        Expression::BinaryOp {
            left,
            right,
            ..
        } => {
            validate_vars_expression(left, vars);
            validate_vars_expression(right, vars);
        },
        Expression::UnaryOp {
            target,
            ..
        } => {
            validate_vars_expression(target, vars);
        },
        Expression::BlockExpression { items, .. } => {
            validate_vars_block(items, vars);
        }
        Expression::Int32Constant { .. } => {},
        Expression::IfExpression { condition, on_true, on_false, .. } => {
            validate_vars_expression(condition, vars);
            validate_vars_expression(on_true, vars);
            if let Some(expression) = on_false {
                validate_vars_expression(expression, vars);
            }
        }
    }
}