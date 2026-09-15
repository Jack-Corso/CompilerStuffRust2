use std::collections::HashMap;
use crate::parsing::{BlockItem, Expression, Program, Statement};
use crate::typing::Type;

pub fn validate_types(ast: &mut Program) {
    let mut ident_types: HashMap<String, Type> = HashMap::new();
    for func in ast.body.iter() {
        assert!(matches!(ident_types.insert(func.name.clone(), func.func_type.clone()), None), "Function '{}' is already defined", func.name);
    }
    for func in ast.body.iter_mut() {
        if let Type::FuncType { return_type, .. } = &func.func_type {
            validate_types_block(&mut func.body, Some(return_type), None, &mut ident_types);
        } else {
            panic!("Function '{}' has illegal type", func.name);
        }
    }
}

fn validate_types_block(items: &mut Vec<BlockItem>, return_type: Option<&Type>, yield_type: Option<&Type>, ident_types: &mut HashMap<String, Type>) {
    for item in items.iter_mut() {
        match item {
            BlockItem::Statement(statement) => {
                validate_types_statement(statement, return_type, yield_type, ident_types);
            },
            BlockItem::VarDeclaration(var_definition) => {
                ident_types.insert(var_definition.name.clone(), var_definition.var_type.clone());
            }
        }
    }
}

fn validate_types_statement(statement: &mut Statement, return_type: Option<&Type>, yield_type: Option<&Type>, ident_types: &mut HashMap<String, Type>) {
    match statement {
        Statement::Expression { expression } => {
            validate_types_expression(expression, &Type::Any, ident_types);
        },
        Statement::Yield { value } => {
            if let Some(expr) = value {
                validate_types_expression(expr, return_type, yield_type.expect("Yield Statement yields unexpected value."), ident_types);
            } else if yield_type.is_some() {
                panic!("Yield Statement value expected.");
            }
        },
        Statement::Return { value } => {
            if let Some(expr) = value {
                validate_types_expression(expr, return_type, return_type.expect("Return Statement returns unexpected value"), ident_types);
            } else if return_type.is_some() {
                panic!("Return Statement value expected.");
            }
        },
        Statement::Block { items } => {
            validate_types_block(items, return_type, None, ident_types);
        }
    }
}

fn validate_types_expression(expression: &mut Expression, return_type: Option<&Type>, expected_type: &Type, ident_types: &mut HashMap<String, Type>) {
    match expression {
        Expression::BlockExpression { items, expr_type } => {
            validate_types_block(items, return_type, Some(expr_type), ident_types);
        },
        Expression::Var { expr_type, name } => {
            let casted = ident_types.get(name).unwrap().try_cast(expected_type);

            if casted.is_none() {
                panic!("Expected type {} but got {}.", expected_type, expr_type);
            } else {
                *expr_type = casted.unwrap();
            }
        },
        Expression::VarAssignment { expr_type, name, value } => {
            let type_ref = ident_types.get(name).unwrap();

            validate_types_expression(value, return_type, type_ref, ident_types);
            let casted = type_ref.try_cast(expected_type);
            if casted.is_none() {
                panic!("Expected type {} but got {}.", expected_type, expr_type);
            } else {
                *expr_type = casted.unwrap();
            }
        },
        Expression::Int32Constant { value, expr_type } => {
            let casted = expr_type.try_cast(expected_type);
            if casted.is_none() {
                panic!("Expected type {} but got {}.", expected_type, expr_type);
            } else {
                *expr_type = casted.unwrap();
            }
        },
        Expression::UnaryOp { expr_type, target, operator } => {
            if matches!(expr_type, Type::Unknown) {
                if let Expression::Var { expr_type: var_type, name } = target.as_mut() {
                    *var_type = ident_types.get(name).unwrap().clone();
                    *expr_type = var_type.clone();
                } else {
                   validate_types_expression(target, return_type, expr_type, ident_types);
                }
            } else {
                if !expr_type.is_convertable_to(expected_type) {
                    panic!("Expected type {} but got {}.", expected_type, expr_type);
                }
            }
        },
        Expression::BinaryOp { expr_type, operator, left, right } => {
            match expr_type {
                Type::Unknown => {

                }
                _ => unreachable!()
            }
        }
    }
}

fn solve_expression_types(expression: &mut Expression, ident_types: &mut HashMap<String, Type>) {
    match expression {
        Expression::UnaryOp { target, operator, expr_type } => {
            if matches!(expr_type, Type::Unknown) {
                solve_expression_types(target, ident_types);
                *expr_type = target.get_type();
            }
        },
        Expression::BinaryOp { left, operator, right, expr_type } => {
            if matches!(expr_type, Type::Unknown) {
                solve_expression_types(left, ident_types);
                solve_expression_types(right, ident_types);
                if left.get_type_ref().is_convertable_to(&right_type) {
                    left.set_type(left.get_type_ref().try_cast(right.get_type_ref()).unwrap());
                } else if right_type.is_convertable_to(&left_type) {
                    right.set_type(right.get_type_ref().try_cast(left.get_type_ref()).unwrap());
                    todo!();
                    right_type.try_cast(&left_type).unwrap()
                } else {
                    panic!("Operator '{operator}' cannot be applied to {left_type} and {right_type}");
                }
            }
        },
        Expression::Var { expr_type, name } => {
            if !matches!(expr_type, Type::Unknown) {
                return expr_type.clone();
            } else {
                return ident_types.get(name).unwrap().clone();
            }
        },
        Expression::VarAssignment { expr_type, name, value } => {
            if !matches!(expr_type, Type::Unknown) {
                return expr_type.clone();
            }

        }
        Expression::BlockExpression { expr_type, .. } | Expres
    }
}

