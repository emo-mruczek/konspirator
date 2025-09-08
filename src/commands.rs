/* commands */

use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::{HashMap, HashSet};
use crate::compiler::Compiler;
use crate::ast::*;
use crate::helpers::Variable;

impl Compiler {

    
    pub fn command_assign(id: &Identifier, expression: &Expression, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::get_variable(id, stack));

        res.push(PUT {pos: G});

        res.extend(Self::handle_expression(expression, initialized, stack));

        res.push(STORE {pos: G});
        
        initialized.insert(Self::get_name(id)); 

        return res;
    }

    pub fn command_read(id: &Identifier, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {

        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::get_variable(id, stack));

        res.push(PUT {pos: H});
        res.push(READ);
        res.push(STORE {pos: H});

        initialized.insert(Self::get_name(&id)); 

        return res;
    }

    pub fn command_write(val: &Value, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(val, stack));

        res.push(WRITE);
        return res;
    }

    pub fn command_if(cond: &Condition, comm: &Vec<Command>, else_comm: &Option<Vec<Command>>, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        let mut block_instructions: Vec<Instruction> = vec![];
        let mut else_block_instructions: Vec<Instruction> = vec![];

        block_instructions.extend(Self::handle_commands(comm, initialized, stack));

        match else_comm {
            Some(commands) => else_block_instructions.extend(Self::handle_commands(commands, initialized, stack)),
            None => {},
        }

        match cond {
            Condition::Equal {l, r} => {
                res.extend(Self::for_handle_equal(l, r, stack, &block_instructions, &else_block_instructions));
            },
            Condition::NotEqual {l, r} => {
                res.extend(Self::for_handle_notequal(l, r, stack, &block_instructions, &else_block_instructions));
            },
            Condition::Greater {l, r} => {
                res.extend(Self::for_handle_greater(l, r, stack, &block_instructions, &else_block_instructions));
            },
            Condition::Less {l, r} => {
                res.extend(Self::for_handle_less(l, r, stack, &block_instructions, &else_block_instructions));
            },
            Condition::GreaterEqual {l, r} => {
                res.extend(Self::for_handle_greaterequal(l, r, stack, &block_instructions, &else_block_instructions));
            },
            Condition::LessEqual {l, r} => {
                res.extend(Self::for_handle_lessequal(l, r, stack, &block_instructions, &else_block_instructions));
            },
        }
        
        return res;
    }
    
    pub fn command_while(cond: &Condition, comm: &Vec<Command>, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        let mut block_instructions: Vec<Instruction> = vec![];
        block_instructions.extend(Self::handle_commands(comm, initialized, stack));

        match cond {
            Condition::Equal {l, r} => {
                res.extend(Self::while_handle_equal(l, r, stack, &block_instructions));
            },
            Condition::NotEqual {l, r} => {
                res.extend(Self::while_handle_notequal(l, r, stack, &block_instructions));
            },
            Condition::Greater {l, r} => {
                res.extend(Self::while_handle_greater(l, r, stack, &block_instructions));
            },
            Condition::Less {l, r} => {
                res.extend(Self::while_handle_less(l, r, stack, &block_instructions));
            },
            Condition::GreaterEqual {l, r} => {
                res.extend(Self::while_handle_greaterequal(l, r, stack, &block_instructions));
            },
            Condition::LessEqual {l, r} => {
                res.extend(Self::while_handle_lessequal(l, r, stack, &block_instructions));
            },
        }

        res.extend(block_instructions);
        res.push(JUMP {pos: -(res.len() as i64), adjust: true});

        return res;
    }
    
    pub fn command_repeat(cond: &Condition, comm: &Vec<Command>, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        let mut block_instructions: Vec<Instruction> = vec![];
        block_instructions.extend(Self::handle_commands(comm, initialized, stack));

        match cond {
            Condition::Equal {l, r} => {
                res.extend(Self::while_handle_equal(l, r, stack, &block_instructions));
            },
            Condition::NotEqual {l, r} => {
                res.extend(Self::while_handle_notequal(l, r, stack, &block_instructions));
            },
            Condition::Greater {l, r} => {
                res.extend(Self::while_handle_greater(l, r, stack, &block_instructions));
            },
            Condition::Less {l, r} => {
                res.extend(Self::while_handle_less(l, r, stack, &block_instructions));
            },
            Condition::GreaterEqual {l, r} => {
                res.extend(Self::while_handle_greaterequal(l, r, stack, &block_instructions));
            },
            Condition::LessEqual {l, r} => {
                res.extend(Self::while_handle_lessequal(l, r, stack, &block_instructions));
            },
        }

        res.extend(block_instructions);

        return res;
    }
}
