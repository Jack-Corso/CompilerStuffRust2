use std::fmt::{Display, Formatter};
use std::io::Write;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use paste::paste;
use strum::VariantArray;
use crate::parsing::Program;
use crate::stack::StackAddr;
use strum_macros::VariantArray;

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

    (enum_def: $name: ident { $($vals:tt)* } + [$($letter:ident),*] + [$($letter_no_h:ident),*]nh + [$($letter_num:ident: [$($num:literal)*,]),*]num) => {
            paste::item! {
                #[derive(Copy, Clone, Eq, PartialEq, strum_macros::EnumString, strum_macros::VariantArray)]
                enum Register {
                    $($vals)*
                    $(

                        [<R $letter X>],
                        [<E $letter X>],
                        [< $letter X>],
                        [< $letter L>],
                        [< $letter H>],
                    )*
                    $(
                        [<R $letter_no_h >],
                        [<E $letter_no_h >],
                        [< $letter_no_h >],
                        [< $letter_no_h L>],
                    )*
                    $(
                        $(
                        [< $letter $num >],
                        [< $letter $num D>],
                        [< $letter $num W>],
                        [< $letter $num B>],
                        )*

                    )*
                }
            }
    };

}

expand_registers!(enum_def:
    Register {

    } + [A, B, C, D] + [SI, DI]nh + [R: [0,2]]num
);


impl Register {
    const HIGH_SIZE: usize = 5;
    const LOW_SIZE: usize = 4;
    const HIGH_VIEW_CUTOFF: usize = Self::HIGH_SIZE * 4;
    const LOW_VIEW_CUTOFF: usize = Self::HIGH_VIEW_CUTOFF + Self::LOW_SIZE * 2;

    fn as_64_bit(&self) -> Register {
        self.as_nth(0)
    }

    fn as_32_bit(&self) -> Register {
        self.as_nth(1)
    }

    fn as_16_bit(&self) -> Register {
        self.as_nth(2)
    }

    fn as_8_bit(&self) -> Register {
        self.as_nth(3)
    }

    fn as_8_bit_high(&self) -> Register {
        if !self.has_8_bit_high() {
            panic!("Register: {self} does not have a high 8 bit view.");
        }
        self.as_nth(4)
    }

    fn has_8_bit_high(&self) -> bool {
        match (self) {
            expand_registers!(pat_branch: SI) | expand_registers!(pat_branch: DI) => false,
            _ => true
        }
    }

    fn as_nth(&self, n: usize) -> Register {
        let num = *self as usize;
        return if num < Self::HIGH_VIEW_CUTOFF {
            Self::VARIANTS[num - (num % Self::HIGH_SIZE)]
        } else {
            Self::VARIANTS[num - ((num - Self::HIGH_VIEW_CUTOFF) % Self::LOW_SIZE)]
        };
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