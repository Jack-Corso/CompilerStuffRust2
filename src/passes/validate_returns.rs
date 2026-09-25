use crate::parsing::{BlockItem, Expression, Program, Statement};

pub fn validate_returns(ast: &Program) {
    for func in ast.body.iter() {
        if !validate_block(&func.body) {
            panic!("Function '{}' does not return in every possible branch", func.name);
        }
    }
}

fn validate_block(block_items: &Vec<BlockItem>) -> bool {
    for block_item in block_items {
        match block_item {
            BlockItem::Statement(statement) => {
                if validate_statement(statement) {
                    return true;
                }
            },
            BlockItem::VarDeclaration(..) => {}
        }
    }
    return false;
}

fn validate_statement(statement: &Statement) -> bool{
    return match statement {
        Statement::Return { .. } => true,
        Statement::Block { items } => {
            validate_block(items)
        },
        Statement::Expression { expression: expr } if let Expression::BlockExpression { items, .. } = &**expr => {
            validate_block(items)
        },
        Statement::Expression { .. } => false,
        Statement::Yield { .. } => false,
        Statement::If { on_true, on_false: Some(statement), .. } =>
            validate_statement(statement) && validate_statement(on_true),
        Statement::If { .. } => false,
    };
}