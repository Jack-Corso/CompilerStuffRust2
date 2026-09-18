use std::arch::x86_64::_bittestandset64;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use std::fs::{File, ReadDir};
use std::io;
use std::io::BufWriter;
use paste::paste;
use strum::VariantArray;
use crate::parsing::{BlockItem, Expression, Function, Program, Statement};
use crate::stack::{StackAddr, StackFrame};
use strum_macros::VariantArray;
use crate::assembly::Register::EAX;
use crate::stack;
#[derive(Debug)]
enum Instruction {
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
    StackSetup {
        size: usize,
        func_name: String,
    },
    StackCleanup {
        size: usize,
        return_label: String,
    },
}

impl Instruction {

    fn move_if_needed(src: &Value, dest: &Value, out: &mut BufWriter<File>) -> io::Result<()>  {
        if src != dest {
            writeln!(out, "\tmovl {src}, {dest}")?;
        }
        Ok(())
    }
    pub fn write_to(&self, out: &mut BufWriter<File>) -> io::Result<()> {
        match self {
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
                    if matches!(src, Value::Address( .. )) && matches!(dest, Value::Address( .. )) {
                        // needs intermediate
                        writeln!(out, "\tmovl {src}, %ecx")?;
                        writeln!(out, "\tmovl %ecx, {dest}")?;
                    } else {
                        writeln!(out, "\tmovl {src}, {dest}")?;
                    }

                }
            },
            Instruction::Unary { operator, target, dest } => {
                if target != dest {
                    writeln!(out, "\tmovl {target}, {dest}")?;
                }
                match operator {
                    UnaryOperator::Negate => writeln!(out, "\tneg {dest}")?,
                    UnaryOperator::Compliment => writeln!(out, "\tnot {dest}")?,
                    UnaryOperator::Not => {
                        writeln!(out, "\tcmpl $0, {dest}")?;
                        // clear dest
                        writeln!(out, "\tmovl $0, {dest}")?;
                        let low_byte_view = match dest {
                            Value::Register( val ) => Value::Register(val.as_1_byte()),
                            // address views should be one time use, this one exits scope immediately, so its ok
                            Value::Address( addr ) => Value::Address(addr.get_n_byte_view(1)),
                            Value::Int32Literal( .. ) => panic!("Invalid Destination Register")
                        };
                        writeln!(out, "\tsete {low_byte_view}")?;
                    }
                }
            },
            Instruction::StackSetup { size, func_name } => {
                writeln!(out, "\t.globl _{func_name}")?;
                writeln!(out, "_{func_name}:")?;
                writeln!(out, "\tpushq %rbp")?;
                writeln!(out, "\tmovq %rsp, %rbp")?;
                writeln!(out, "\tsubq ${size}, %rsp")?;
            },
            Instruction::StackCleanup { size, return_label } => {
                writeln!(out, "{return_label}:")?;
                writeln!(out, "\tmovq %rbp, %rsp")?;
                writeln!(out, "\tpopq %rbp")?;
                writeln!(out, "\tret")?;
            }
            Instruction::Label { label } => {
                writeln!(out, "{label}:")?;
            },
            Instruction::Binary { operator, left, right, dest } => {
                if *operator == BinaryOperator::Divide {
                    Self::move_if_needed(left, &Value::Register(Register::EAX), out)?;
                    writeln!(out, "\tcdq")?;
                    writeln!(out, "\tidiv {right}")?;
                    Self::move_if_needed(&Value::Register(Register::EAX), dest, out)?;
                } else {
                    Self::move_if_needed(left, dest, out)?;
                    macro_rules! cmp_and_clear {
                        () => {
                            {
                                writeln!(out, "\tcmpl {right}, {dest}")?;
                                writeln!(out, "movl $0, {dest}")?;
                                dest.get_n_byte_view(1)
                            }
                        };
                    }
                    match operator {
                        BinaryOperator::Add => writeln!(out, "\taddl {right}, {dest}")?,
                        BinaryOperator::Subtract => writeln!(out, "\tsubl {right}, {dest}")?,
                        BinaryOperator::Multiply => writeln!(out, "\timull {right}, {dest}")?,

                        BinaryOperator::Equal => {
                            // technically an extra move idc tho lowkey
                            let low_view = cmp_and_clear!();
                            writeln!(out, "\tsete {low_view}")?
                        },
                        BinaryOperator::NotEqual => {
                            // technically an extra move idc tho lowkey
                            let low_view = cmp_and_clear!();
                            writeln!(out, "\tsetne {low_view}")?
                        },
                        BinaryOperator::GreaterThan => {
                            // technically an extra move idc tho lowkey
                            let low_view = cmp_and_clear!();
                            writeln!(out, "\tsetg {low_view}")?
                        },
                        BinaryOperator::GreaterOrEqual => {
                            let low_view = cmp_and_clear!();
                            writeln!(out, "\tsetge {low_view}")?
                        },
                        BinaryOperator::LessThan => {
                            let low_view = cmp_and_clear!();
                            writeln!(out, "\tsetl {low_view}")?
                        },
                        BinaryOperator::LessOrEqual => {
                            let low_view = cmp_and_clear!();
                            writeln!(out, "\tsetle {low_view}")?
                        },
                        BinaryOperator::Divide => unreachable!(),
                    }
                }

            }
        }
        Ok(())
    }
}

macro_rules! reg {
    ($name: ident) => {
        Value::Register(Register::$name)
    }
}



#[derive(Eq, PartialEq, Debug)]
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

impl Value {
    pub fn get_n_byte_view(&self, num_bytes: usize) -> Value {
        match self {
            Value::Register( register ) => Value::Register(register.as_n_bytes(num_bytes as u8)),
            Value::Address(addr) => Value::Address(addr.get_n_byte_view(num_bytes)),
            _ => panic!("Cannot create {num_bytes} byte view of {self}")
        }
    }

    pub fn is_storage(&self) -> bool {
        matches!(self, Value::Register( .. ) | Value::Address( .. ))
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum UnaryOperator {
    Compliment,
    Negate,
    Not
}

impl From<&String> for UnaryOperator {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "-" => UnaryOperator::Negate,
            "!" => UnaryOperator::Not,
            "~" => UnaryOperator::Compliment,
            _ => panic!("Invalid UnaryOperator")
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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

impl From<&String> for BinaryOperator {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "+" => BinaryOperator::Add,
            "-" => BinaryOperator::Subtract,
            "*" => BinaryOperator::Multiply,
            "/" => BinaryOperator::Divide,
            "==" => BinaryOperator::Equal,
            "!=" => BinaryOperator::NotEqual,
            "<" => BinaryOperator::LessThan,
            "<=" => BinaryOperator::LessOrEqual,
            ">" => BinaryOperator::GreaterThan,
            ">=" => BinaryOperator::GreaterOrEqual,
            _ => panic!("Invalid BinaryOperator")
        }
    }
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
                #[derive(Copy, Clone, Eq, PartialEq, Debug, strum_macros::EnumString, strum_macros::VariantArray)]
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

                }
            }
    };

}

expand_registers!(enum_def:
    Register {

    } + [A, B, C, D] + [SI, DI]nh
);


impl Register {
    const HIGH_SIZE: usize = 5;
    const LOW_SIZE: usize = 4;
    const HIGH_VIEW_CUTOFF: usize = Self::HIGH_SIZE * 4;
    const LOW_VIEW_CUTOFF: usize = Self::HIGH_VIEW_CUTOFF + Self::LOW_SIZE * 2;


    fn as_8_byte(&self) -> Register {
        self.as_nth(0)
    }

    fn as_4_byte(&self) -> Register {
        self.as_nth(1)
    }

    fn as_2_byte(&self) -> Register {
        self.as_nth(2)
    }

    fn as_1_byte(&self) -> Register {
        self.as_nth(3)
    }

    fn as_n_bytes(&self, num_bytes: u8) -> Register {
        match (num_bytes) {
            1 => self.as_1_byte(),
            2 => self.as_2_byte(),
            4 => self.as_4_byte(),
            8 => self.as_8_byte(),
            _ => panic!("Cannot get view of size {num_bytes} bits")
        }
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
        let mut string = format!("{:?}", self);
        string.make_ascii_lowercase();
        write!(f, "%{string}")

    }
}


pub fn generate_asm(ast: Program, out: &mut BufWriter<File>) -> io::Result<()> {
    let tacky = create_tacky(ast);
    println!("{}", tacky.iter().map(|i| format!("{i:?}")).collect::<Vec<String>>().join("\n"));

    writeln!(out, "\t.globl WinMain")?;
    writeln!(out, "WinMain:")?;
    writeln!(out, "\tcall _main")?;
    writeln!(out, "\tret")?;

    for instruction in tacky {
        instruction.write_to(out)?;
    }

    Ok(())
}

struct LabelManager {
    label_counts: HashMap<String, usize>
}

impl LabelManager {
    fn new() -> LabelManager {
        LabelManager { label_counts: HashMap::new() }
    }

    fn gen_ret_label(&mut self) -> String {
        self.gen_label("return")
    }

    fn gen_label(&mut self, name: &str) -> String {
        if (!self.label_counts.contains_key(name)) {
            self.label_counts.insert(String::from(name), 0);
        } else {
            self.label_counts.insert(String::from(name), self.label_counts[name]+1);
        }
        self.last_label(name)
    }

    fn has_label(&self, name: &str) -> bool {
        self.label_counts.contains_key(name)
    }

    fn last_label(&self, name: &str) -> String {
        format!("{name}{}", self.label_counts[name])
    }

    fn last_ret_label(&self) -> String {
        self.last_label("return")
    }
}

fn create_tacky(ast: Program) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut label_manager = LabelManager::new();
    for func in ast.body {
        create_tacky_func(&func, &mut instructions, &mut label_manager);
    }
    return instructions;

}

fn create_tacky_func(func: &Function, instructions: &mut Vec<Instruction>, label_manager: &mut LabelManager) {
    // semi-expensive
    let mut stack = stack::create_stack_frame(func);
    instructions.push(
        Instruction::StackSetup {
            func_name: func.name.clone(),
            size: stack.size()
        }
    );

    let ret_label = label_manager.gen_ret_label();

    create_tacky_block(&func.body, instructions, &mut stack, label_manager, true);

    instructions.push(
        Instruction::StackCleanup {
            return_label: ret_label,
            size: stack.size()
        }
    );
}

fn create_tacky_block(
    block_items: &Vec<BlockItem>,
    instructions: &mut Vec<Instruction>, // out
    stack_frame: &mut StackFrame,
    label_manager: &mut LabelManager,
    is_func: bool
) {
    let end_label;
    if !is_func {
        end_label = label_manager.gen_label("block_end");
    } else {
        end_label = String::new();
    }
    let mut variables: Vec<&String> = Vec::new();
    for block_item in block_items.iter() {
        match block_item {
            BlockItem::Statement(statement) => create_tacky_statement(statement, instructions, stack_frame, label_manager, &end_label),
            BlockItem::VarDeclaration(var_declaration) => {
                variables.push(&var_declaration.name);
                stack_frame.reserve_var(var_declaration.name.clone(), var_declaration.var_type.get_size());
                if var_declaration.init_value.is_some() {
                    // I could prob optimize out the copy here but idc
                    create_tacky_expression(var_declaration.init_value.as_ref().unwrap(), instructions, stack_frame, label_manager, Some(Value::Register(EAX)));

                    instructions.push(Instruction::Copy { src: reg!(EAX), dest: Value::Address(*stack_frame.get_var(&var_declaration.name)) })
                }
            }
        }
    }
    if !is_func {
        instructions.push(Instruction::Label { label: end_label })
    }
    for var_name in variables {
        stack_frame.free_var(var_name);
    }
}

fn create_tacky_statement(
    statement: &Statement,
    instructions: &mut Vec<Instruction>, // out
    stack_frame: &mut StackFrame,
    label_manager: &mut LabelManager,
    block_end: &String,
) {
    match statement {
        Statement::Return { value } => {
            if let Some(expr) = value {
                create_tacky_expression(expr, instructions, stack_frame, label_manager, Some(reg!(EAX)));
            }
            instructions.push(Instruction::Jump {dest: label_manager.last_ret_label()});
        },
        Statement::Block { items } => {
            create_tacky_block(items, instructions, stack_frame, label_manager, false);
        },
        Statement::Expression { expression } => {
            create_tacky_expression(expression, instructions, stack_frame, label_manager, None);
        },
        Statement::Yield { value } => {
            if let Some(expr) = value {
                create_tacky_expression(expr, instructions, stack_frame, label_manager, Some(reg!(EAX)));
            }
            instructions.push(Instruction::Jump {dest: block_end.clone()});
        }
    }
}

fn create_tacky_expression(
    expression: &Expression,
    instructions: &mut Vec<Instruction>, // out
    stack_frame: &mut StackFrame,
    label_manager: &mut LabelManager,
    dest: Option<Value>
) {
    let dest = dest.unwrap_or_else(|| reg!(EAX));
    match expression {
        Expression::Var { name, .. } => {
            instructions.push(Instruction::Copy { src: Value::Address(*stack_frame.get_var(name)), dest });
        },
        Expression::Int32Constant { value, .. } => {
            instructions.push(Instruction::Copy { src: Value::Int32Literal(*value), dest });
        },
        Expression::VarAssignment { name, value, .. } => {
            // yet another prob removable copy
            create_tacky_expression(value, instructions, stack_frame, label_manager, Some(reg!(EAX)));
            instructions.push(Instruction::Copy { src: reg!(EAX), dest: Value::Address(*stack_frame.get_var(name)) });
            // move value to desired dest too
            instructions.push(Instruction::Copy { src: reg!(EAX), dest });
        },
        Expression::UnaryOp { target, operator, .. } => {
            create_tacky_expression(target, instructions, stack_frame, label_manager, Some(reg!(EAX)));
            instructions.push(Instruction::Unary {
                target: reg!(EAX),
                operator: UnaryOperator::from(operator),
                dest
            });
        },
        Expression::BinaryOp { operator, left, right, .. }  => {
            let temp = stack_frame.reserve(4);
            create_tacky_expression(right, instructions, stack_frame, label_manager, Some(Value::Address(temp)));
            create_tacky_expression(left, instructions, stack_frame, label_manager, Some(reg!(EAX)));
            instructions.push(Instruction::Binary {
                operator: BinaryOperator::from(operator),
                left: reg!(EAX),
                right: Value::Address(temp),
                dest,
            });

            stack_frame.free(temp);
        },
        Expression::BlockExpression { items, .. } => {
            create_tacky_block(items, instructions, stack_frame, label_manager, false);
        }
    }
}
