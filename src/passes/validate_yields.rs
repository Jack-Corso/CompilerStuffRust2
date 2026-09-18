use crate::parsing::{BlockItem, Expression, Program, Statement};

pub fn validate_returns(ast: &Program) {
    for func in ast.body.iter() {
        if !validate_block(&func.body, false) {
            panic!("Yield outside of block in function '{}'", func.name);
        }
    }
}

fn validate_block(block_items: &Vec<BlockItem>, in_block: bool) -> bool {
    for block_item in block_items {
        match block_item {
            BlockItem::Statement(statement) => {
                if validate_statement(statement, in_block) {
                    if in_block {
                        return true;
                    }
                } else if !in_block {
                    panic!("Yield outside of block");
                }
            },
            BlockItem::VarDeclaration(..) => {}
        }
    }
    return false;
}

fn validate_statement(statement: &Statement, in_block: bool) -> bool{
    return match statement {
        Statement::Return { .. } => !in_block,
        Statement::Block { items } => {
            validate_block(items, true)
        },
        Statement::Expression { expression: expr } if let Expression::BlockExpression { items, .. } = &**expr => {
            validate_block(items, true)
        },
        Statement::Expression { .. } => !in_block,
        Statement::Yield { .. } => in_block,
    };
}
