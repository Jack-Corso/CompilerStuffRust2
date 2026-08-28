use std::fs::File;
use std::io::BufWriter;
use crate::parsing::Program;
use crate::stack::StackAddr;

enum Instruction {
    Return {
        value: Value
    },
    Unary {
        operator: UnaryOperator,
        target: Value,
        dest: Value,
    },
    Binary {
        operator: BinaryOperator,
        left: Value,
        right: Value,
        dest: Value,
    },
    Copy {
        src: Value,
        dest: Value,
    },
    Jump {
        dest: String
    },
    JumpIfZero {
        condition: Value,
        dest: String,
    },
    JumpNotZero {
        condition: Value,
        dest: String,
    },
    Label {
        label: String,
    },
}

impl Instruction {
    fn write_to(&self, out: BufWriter<File>) {
        match self {
            Instruction::Return { value } => {

            }
        }
    }
}

enum Value {
    Int32Literal(i32),
    Address(StackAddr),
    Register(Register)
}

enum UnaryOperator {
    Compliment,
    Negate,
    Not
}

enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
}

enum Register {
    EAX,
    EDX,
}

pub fn generate_asm(ast: Program, out: BufWriter<File>) {

}

fn create_tacky(ast: Program) -> Vec<Instruction> {

}