use std::collections::HashMap;
use std::process::id;
use crate::parsing::{BlockItem, Expression, Program, Statement};
use crate::typing::Type;

pub fn validate_types(ast: &mut Program) {

    for func in ast.body.iter_mut() {
        if let Type::FuncType { return_type, .. } = &func.func_type {
            validate_types_block(&mut func.body, Some(return_type), None);
        } else {
            panic!("Function '{}' has illegal type", func.name);
        }
    }
}

fn validate_types_block(items: &mut Vec<BlockItem>, return_type: Option<&Type>, yield_type: Option<&Type>) {
    for item in items.iter_mut() {
        match item {
            BlockItem::Statement(statement) => {
                validate_types_statement(statement, return_type, yield_type);
            },
            BlockItem::VarDeclaration(var_definition) => {
                if let Some(expr) = var_definition.init_value.as_mut() {
                    validate_types_expression(expr, return_type, &var_definition.var_type);
                }
            }
        }
    }
}

fn validate_types_statement(statement: &mut Statement, return_type: Option<&Type>, yield_type: Option<&Type>) {
    match statement {
        Statement::Expression { expression } => {
            validate_types_expression(expression, return_type, &Type::Any);
        },
        Statement::Yield { value } => {
            if yield_type.is_some() {
                if let Some(expression) = value {
                    validate_types_expression(expression, return_type,yield_type.unwrap());
                } else {
                    panic!("Expected yield to return a value");
                }
            } else if value.is_some() {
                panic!("Unexpected yield value")
            }
        },
        Statement::Return { value } => {
            if return_type.is_some() {
                if let Some(expression) = value {
                    validate_types_expression(expression, return_type, return_type.unwrap());
                } else {
                    panic!("Expected return to return a value");
                }
            } else if value.is_some() {
                panic!("Unexpected return value")
            }
        },
        Statement::Block { items } => {
            validate_types_block(items, return_type, None);
        }
    }
}

fn validate_types_expression(expression: &mut Expression, return_type: Option<&Type>, expected_type: &Type) {

}

