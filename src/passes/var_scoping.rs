use std::collections::{HashMap, HashSet, VecDeque};
use crate::parsing::{BlockItem, Expression, Function, Program, Statement};

pub fn scope_vars(ast: &mut Program) {
    for func in ast.body.iter_mut() {
        scope_vars_block(&mut func.body, 0, &mut HashMap::new());
    }
}

fn scope_vars_block(block_items: &mut Vec<BlockItem>, scope_level: u32, aliases: &mut HashMap<String, Vec<String>>) {
    let mut defined_vars: Vec<String> = Vec::new();

    for block_item in block_items {
        match block_item {
            BlockItem::VarDeclaration( var_declaration ) => {
                if !aliases.contains_key(&var_declaration.name) {
                    aliases.insert(var_declaration.name.clone(), Vec::new());
                }
                let alias = format!("{}:{scope_level}", var_declaration.name);
                aliases.get_mut(&var_declaration.name).unwrap().push(alias.clone());
                defined_vars.push(var_declaration.name.clone());
                var_declaration.name.replace_range(..var_declaration.name.len(), alias.as_str());
            },
            BlockItem::Statement( statement ) => {
                scope_vars_statement(statement, scope_level, aliases);
            }
        }
    }

    for var in defined_vars {
        aliases.get_mut(&var).unwrap().pop();
    }
}

fn scope_vars_statement(statement: &mut Statement, scope_level: u32, aliases: &mut HashMap<String, Vec<String>>) {
    match statement {
        Statement::Expression { expression } => {
            scope_vars_expression(expression, aliases, scope_level);
        },
        Statement::Block { items } => {
            scope_vars_block(items, scope_level + 1, aliases);
        },
        Statement::Return { value } | Statement::Yield { value } => {
            if let Some(expr) = value {
                scope_vars_expression(expr, aliases, scope_level);
            }
        },
    }
}

fn scope_vars_expression(expression: &mut Expression, aliases: &mut HashMap<String, Vec<String>>, scope_level: u32) {
    match expression {
        Expression::Var { name, .. } => {
            name.replace_range(..name.len(), aliases.get(name).unwrap().last().unwrap());
        },
        Expression::VarAssignment { name, value, .. } => {
            name.replace_range(..name.len(), aliases.get(name).unwrap().last().unwrap());
            scope_vars_expression(value, aliases, scope_level);
        },
        Expression::UnaryOp { target, .. } => {
            scope_vars_expression(target, aliases, scope_level);
        },
        Expression::BinaryOp {
            left,
            right,
            ..
        } => {
            scope_vars_expression(left, aliases, scope_level);
            scope_vars_expression(right, aliases, scope_level);
        },
        Expression::BlockExpression { items, .. } => {
            scope_vars_block(items, scope_level + 1, aliases);
        }
        Expression::Int32Constant { .. } => {}
    }
}