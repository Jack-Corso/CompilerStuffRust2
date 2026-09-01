use std::fmt::{Display, Formatter};
use std::io::Write;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use paste::paste;
use crate::parsing::Program;
use crate::stack::StackAddr;
use strum_macros::Display;

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
        strum::Into
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
macro_rules! as_item {
    ($i:item) => { $i };
}
macro_rules! expand_registers {
    (pat_branch_h: $letter:ident) => {
        paste! {
            Register::[<R $letter X>]
                | Register::[<E $letter X>]
                | Register::[< $letter X>]
                | Register::[< $letter H>]
                | Register::[< $letter L>]
        }
    };
    (pat_branch: $letter:ident) => {
        paste! {
            Register::[<R $letter >]
                | Register::[<E $letter >]
                | Register::[< $letter >]
                | Register::[< $letter L>]
        }
    };

    (enum_def: $name: ident { $($vals:tt)* } + [$($letter:ident),*] + [$($letter_no_h:ident),*]nh) => {
            paste::item! {
                #[derive(Eq, PartialEq, strum_macros::EnumString)]
                enum Register {
                    $($vals)*
                    $(

                        [<R $letter X>],
                        [<E $letter X>],

                        [< $letter X>],

                        [< $letter H>],
                        [< $letter L>],
                    )*
                    $(
                        [<R $letter_no_h >],
                        [<E $letter_no_h >],
                        [< $letter_no_h >],
                        [< $letter_no_h L>],
                    )*
                }
            }
    };

}

expand_registers!(enum_def:
    Register {

    } + [A, B, C, D] + [SI, DI]nh
);


impl Register {

    fn as_64_bit(&self) -> Register {
        match (self) {
            expand_registers!(pat_branch_h: A) => Register::RAX,
            expand_registers!(pat_branch_h: B) => Register::RBX,
            expand_registers!(pat_branch_h: C) => Register::RCX,
            expand_registers!(pat_branch_h: D) => Register::RDX,
            expand_registers!(pat_branch: SI) => Register::RSI,
            expand_registers!(pat_branch: DI) => Register::RDI,
        }
    }

    fn as_32_bit(&self) -> Register {
        match (self) {
            expand_registers!(pat_branch_h: A) => Register::EAX,
            expand_registers!(pat_branch_h: B) => Register::EBX,
            expand_registers!(pat_branch_h: C) => Register::ECX,
            expand_registers!(pat_branch_h: D) => Register::EDX,
            expand_registers!(pat_branch: SI) => Register::ESI,
            expand_registers!(pat_branch: DI) => Register::EDI,
        }
    }

    fn as_16_bit(&self) -> Register {
        match (self) {
            expand_registers!(pat_branch_h: A) => Register::AX,
            expand_registers!(pat_branch_h: B) => Register::BX,
            expand_registers!(pat_branch_h: C) => Register::CX,
            expand_registers!(pat_branch_h: D) => Register::DX,
            expand_registers!(pat_branch: SI) => Register::SI,
            expand_registers!(pat_branch: DI) => Register::DI,
        }
    }

    fn as_8_bit(&self) -> Register {
        match (self) {
            expand_registers!(pat_branch_h: A) => Register::AL,
            expand_registers!(pat_branch_h: B) => Register::BL,
            expand_registers!(pat_branch_h: C) => Register::CL,
            expand_registers!(pat_branch_h: D) => Register::DL,
            expand_registers!(pat_branch: SI) => Register::SIL,
            expand_registers!(pat_branch: DI) => Register::DIL,
        }
    }

    fn as_8_bit_high(&self) -> Register {
        match (self) {
            expand_registers!(pat_branch_h: A) => Register::AH,
            expand_registers!(pat_branch_h: B) => Register::BH,
            expand_registers!(pat_branch_h: C) => Register::CH,
            expand_registers!(pat_branch_h: D) => Register::DH,
            _ => panic!("Register {self} does not have a high byte view")
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = self.to_string();
        string.make_ascii_lowercase();
        write!(f, "%{}", string)
    }
}


pub fn generate_asm(ast: Program, out: BufWriter<File>) {

}

fn create_tacky(ast: Program) -> Vec<Instruction> {

}