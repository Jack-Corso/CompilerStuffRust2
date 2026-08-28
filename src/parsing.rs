use std::collections::VecDeque;
use crate::lexing::Token;
use crate::parsing::Expression::{BinaryOp, VarAssignment};
use crate::typing::Type;

macro_rules! peek_or_ret {
    ($stack: expr, $index: expr) => {
        {
            if ($stack.len() <= $index) { return None; }
            $stack.get($index).unwrap()
        }
    };
    ($expected: expr, $stack: expr, $index: expr) => {
        {
            if ($stack.len() <= $index || $stack[$index].content() != $expected) { return None; }
            $stack.get($index).unwrap()
        }
    }
}

macro_rules! pop_or_panic {
    ($expected: literal, $stack: expr, $($arg:tt)*) => {
       {
           if $stack.is_empty() || $stack[0].content() != $expected { panic!($($arg)*); }
           $stack.pop_front().unwrap()
       }
    };
    ($expected: literal, $stack: expr) => {
        pop_or_panic!($expected, $stack, "Unexpected end of file")
    };
    ($stack: expr, $($arg:tt)*) => {
        {
            if ($stack.is_empty()) { panic!($($arg)*); }
            $stack.pop_front().unwrap()
        }
    };
    ($stack: expr) => {
        pop_or_panic!($stack, "Unexpected end of file")
    };


}
// this could be a func but whatever
macro_rules! pop_mult {
    ($stack: expr, $amount: expr) => {
        for _ in 0..$amount {
            $stack.pop_front().unwrap();
        }
    }
}

pub struct Program {
    pub body: Vec<Function>
}

pub struct Function {
    pub name: String,
    pub body: Vec<BlockItem>,
    pub func_type: Type
}

pub enum Statement {
    Return {
        value: Box<Expression>,
    },
    Block {
        items: Vec<BlockItem>
    },
    Expression {
        expression: Box<Expression>,
    }
}

pub enum Expression {
    UnaryOp {
        operator: String,
        target: Box<Expression>,
    },
    BinaryOp {
        operator: String,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Var {
        name: String,
    },
    VarAssignment {
        name: String,
        value: Box<Expression>,
    },
    Int32Constant {
        value: i32,
    }
}

pub struct VarDeclaration {
    pub name: String,
    pub init_value: Option<Expression>,
    pub var_type: Type,
}

pub enum BlockItem {
    Statement(Statement),
    VarDeclaration(VarDeclaration),
}
type TokenStack = VecDeque<Token>;
pub fn parse(tokens: Vec<Token>) -> Program {
    let mut stack: TokenStack = tokens.into_iter().collect();
    let mut functions: Vec<Function> = Vec::new();

    while let Some(func) = parse_function(&mut stack) {
        functions.push(func);
    };
    if !stack.is_empty() {
        panic!("Unexpected tokens at end of file: {stack:?}")
    }

    Program { body: functions }
}

fn parse_function(tokens: &mut TokenStack) -> Option<Function> {
    peek_or_ret!("func", tokens, 0);
    let name: String;
    if let Token::Identifier(content) = peek_or_ret!(tokens, 1) {
        name = content.clone();
    } else { return None; }

    peek_or_ret!("(", tokens, 2);
    peek_or_ret!(")", tokens, 3);
    pop_mult!(tokens, 4);
    pop_or_panic!("->", tokens, "Expected function to have return type");
    pop_or_panic!("i32", tokens);
    pop_or_panic!("{", tokens, "Expected block after function definition");

    let mut block_items = Vec::new();
    let mut next = tokens.front().expect("Expected \"}\" to close function body");
    while next.content() != "}" {
        block_items.push(parse_block_item(&mut *tokens).expect("Unexpected token sequence"));
        next = tokens.front().expect("Expected \"}\" to close function body");
    }
    tokens.pop_front();

    Some(Function {
        name,
        body: block_items,
        func_type: Type::FuncType {
            param_types: vec![],
            return_type: Box::new(Type::Int32)
        },
    })
}

fn parse_block_item(tokens: &mut TokenStack) -> Option<BlockItem> {
    if let Some(statement) = parse_statement(tokens) {
        return Some(BlockItem::Statement(statement));
    }
    Some(BlockItem::VarDeclaration(parse_var_definition(tokens)?))
}

fn parse_statement(tokens: &mut TokenStack) -> Option<Statement> {
    if let Some(expr) = parse_expression(tokens, 0) {
        pop_or_panic!(";", tokens, "Expected expression to end with ';'");
        return Some(Statement::Expression {expression: Box::new(expr)});
    }
    let current = peek_or_ret!(tokens, 0);
    if current.content() == "return" {
        tokens.pop_front();
        let value = parse_expression(tokens, 0).expect("Expected expression after return");
        pop_or_panic!(";", tokens, "Expected \";\" after return");
        return Some(Statement::Return {
            value: Box::new(value),
        });
    } else if current.content() == "{" {
        tokens.pop_front();
        let mut block_items = Vec::new();
        let mut next = tokens.front().expect("Expected \"}\" to close block");
        while next.content() != "}" {
            block_items.push(parse_block_item(&mut *tokens).expect("Unexpected token sequence"));
            next = tokens.front().expect("Expected \"}\" to close block");
        }
        return Some(Statement::Block {
            items: block_items,
        });
    }
    None
}

fn parse_expression(tokens: &mut TokenStack, min_precedence: usize) -> Option<Expression> {
    let mut left = parse_factor(tokens)?;
    loop {

        let next_token = tokens.front();
        if (next_token.is_none()) {
            return Some(left);
        }

        let next_token = &next_token.unwrap().clone();
        match next_token {
            Token::Operator(operator) if is_binary_op(operator) => {
                let precedence = get_precedence(operator);
                if (precedence > min_precedence) {
                    tokens.pop_front();
                    if (is_assignment_op(operator)) { // right associative
                        let mut right = parse_expression(tokens, precedence).expect("Expected expression after assignment");
                        match (&left) {
                            Expression::Var { name } => {
                                if operator.len() == 2 {
                                    // get first char
                                    let extra_op = operator.strip_suffix("=").unwrap().to_string();
                                    right = Expression::BinaryOp {
                                        operator: extra_op,
                                        left: Box::new(right),
                                        right: Box::new(Expression::Var { name: name.clone() }),
                                    }
                                }
                                left = VarAssignment { name: name.clone(), value: Box::new(right) };
                            },
                            _=> panic!("Expected variable name before assignment var")
                        }
                    } else {
                        let right = parse_expression(tokens, precedence).expect("Expected expression after binary op");
                        left = BinaryOp {
                            operator: operator.clone(),
                            left: Box::new(left),
                            right: Box::new(right),
                        };
                    }
                } else {
                    break;
                }
            },
            _ => break
        };
    };
    return Some(left);

}

fn parse_factor(tokens: &mut TokenStack) -> Option<Expression> {
    let token = peek_or_ret!(tokens, 0).clone();

    return match (token) {
        Token::Int32Literal(content) => {
            tokens.pop_front();
            let value: i32 = content.parse().unwrap();
            Some(Expression::Int32Constant { value })
        },
        Token::Operator(operator) if is_unary_op(&operator) => {
            tokens.pop_front();
            let target = parse_factor(tokens).expect("Expected factor after unary operator");
            Some(Expression::UnaryOp {
                operator: operator.clone(),
                target: Box::new(target),
            })
        },
        Token::Separator(separator) if separator == "(" => {
            tokens.pop_front();
            let expression = parse_expression(tokens, 0).expect("Expected expression after left parenthesis");
            pop_or_panic!(")", tokens, "Expected \")\" after expression");
            Some(expression)
        },
        Token::Identifier(name) => {
            tokens.pop_front();
            let var = Expression::Var { name: name.clone() };
            // handle increment & decrement operators
            if !tokens.is_empty() {
                let next_cont = tokens[0].content();
                if next_cont == "++" || next_cont == "--" {
                    let mut modified_op = String::from(next_cont);
                    modified_op.push_str("r");
                    return Some(Expression::UnaryOp {
                        operator: modified_op,
                        target: Box::new(var),
                    });
                }
            }
            Some(var)
        },
        _ => None
    }
}

fn parse_var_definition(tokens: &mut TokenStack) -> Option<VarDeclaration> {
    peek_or_ret!("let", tokens, 0);
    tokens.pop_front();
    if let Token::Identifier(name) = pop_or_panic!(tokens, "Expected identifier after let") {
        pop_or_panic!(":", tokens, "Expected type annotation for var");
        pop_or_panic!("i32", tokens);
        return if (tokens.front().is_some() && tokens.front().unwrap().content() == "=") {
            tokens.pop_front();
            let init_value = parse_expression(tokens, 0).expect("Expected expression after variable initializer");
            pop_or_panic!(";", tokens, "Expected \";\" after var init");
            Some(VarDeclaration {
                name: name.clone(),
                init_value: Some(init_value),
                var_type: Type::Int32,
            })
        } else {
            pop_or_panic!(";", tokens, "Expected \";\" after var init");
            Some(VarDeclaration {
                name: name.clone(),
                init_value: None,
                var_type: Type::Int32,
            })
        }
    } else {
        panic!("Expected identifier after let");
    }
}

fn is_unary_op(operator: &String) -> bool {
    match operator.as_str() {
        "-" | "!" | "~" | "++" | "--" => true,
        _ => false
    }
}

fn is_binary_op(operator: &String) -> bool {
    (!is_unary_op(operator) || operator == "-") && operator != ":"
}

fn is_assignment_op(operator: &String) -> bool {
    match operator.as_str() {
        "=" | "+=" | "-=" | "*=" | "/=" => true,
        _=> false
    }
}

fn get_precedence(operator: &str) -> usize {
    match (operator) {
        "*" | "/" => 50,
        "+" | "-" => 45,
        "<" | ">" | "<=" | ">=" => 35,
        "==" | "!=" => 30,
        "&&" => 20,
        "||" => 10,
        "?" => 3,
        "=" | "+=" | "-=" | "*=" | "/=" => 1,
        _ => panic!("operator \"{}\" does not have an assigned precedence value", operator)
    }
}