use std::fmt::Display;
use std::io::Write;
use std::fs::File;
use std::io;
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
    fn write_to(&self, out: &mut BufWriter<File>) -> io::Result<()> {

        match self {
            Instruction::Return { value } => {
                writeln!(out, "\tret")?;
            },
            Instruction::JumpIfZero { condition, dest } => {
                writeln!(out, "\tcmpl {condition}, 0")?;
                writeln!(out, "\tje {dest}")?;
            },
            Instruction::JumpNotZero { condition, dest } => {
                writeln!(out, "\tcmpl {condition}, 0")?;
                writeln!(out, "\tjne {dest}")?;
            },
            Instruction::Jump { dest } => {
                writeln!(out, "\tjmp {dest}")?;
            },
            Instruction::Copy { src, dest } => {
                if src != dest {
                    writeln!(out, "\tmovl {src}, {dest}")?;
                }
            },
            Instruction::Unary { operator, target, dest } => {
                match operator {
                    UnaryOperator::Negate => writeln!(out, "\tneg {target}")?;
                    UnaryOperator::Not => e
                }
            }
        }
        Ok(())
    }
}
#[derive(Eq, PartialEq)]
enum Value {
    Int32Literal(i32),
    Address(StackAddr),
    Register(Register)
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int32Literal(x) => write!(f, "${}", x),
            Value::Address(addr) => write!(f, "{addr}"),
            Value::Register(reg) => write!(f, "{reg}"),
        }
    }
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

#[derive(Eq, PartialEq)]
enum Register {
    RAX,
    EAX,
    AX,
    AH,
    AL,
    RDX,
    EDX,
    DX,
    DH,
    DL
}
macro_rules! reg_branch {
    ($letter:tt) => {
        Register::R($letter)X | Register::E$letterX | Register::$letterX | Register::$letterH | Register::$letterL
    }
}
impl Register {

    fn as_64_bit(&self) -> Register {

        match (self) {
            Register::RAX | Register::EAX | Register::AX | Register::AH | Register::AL => Register::RAX,
            Register::RDX | Register::EDX | Register::DX | Register::DH | Register::DL => Register::RDX,
            reg_branch!(A) => Register::RAX

        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Register::EAX => write!(f, "%eax"),
            Register::EDX => write!(f, "%edx"),
        }
    }
}

pub fn generate_asm(ast: Program, out: BufWriter<File>) {

}

fn create_tacky(ast: Program) -> Vec<Instruction> {

}