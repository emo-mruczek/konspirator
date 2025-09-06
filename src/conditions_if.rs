/* condition handling for if */

/* BROKEN!!!!!!!!!!!!!!!!!!!!!!!!! */

use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::HashMap;
use crate::compiler::Compiler;
use crate::ast::*;
use crate::helpers::Variable;

impl Compiler {

    pub fn handle_equal_if(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(r, stack));
        res.push(PUT {pos: C});
        res.push(SUB {pos: B});
        res.push(JPOS {pos: (block_instructions.len() as i64) + 5, adjust: false});
        res.push(GET {pos: B});
        res.push(SUB {pos: C});
        res.push(JPOS {pos: (block_instructions.len() as i64) + 2, adjust: false});
        res.extend(block_instructions.clone());
        res.push(JUMP {pos: (else_block_instructions.len() as i64) + 1, adjust: false});
        res.extend(else_block_instructions.clone());

        return res;
    }

    pub fn handle_notequal_if(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(r, stack));
        res.push(PUT {pos: C});
        res.push(SUB {pos: B});
        res.push(JPOS {pos: (else_block_instructions.len() as i64) + 5, adjust: false});
        res.push(GET {pos: B});
        res.push(SUB {pos: C});
        res.push(JPOS {pos: (else_block_instructions.len() as i64) + 2, adjust: false});
        res.extend(else_block_instructions.clone());
        res.push(JUMP {pos: (block_instructions.len() as i64) + 1, adjust: false});
        res.extend(block_instructions.clone());

        return res;
    }
    
    pub fn handle_greater_if(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(r, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(l, stack));
        res.push(SUB {pos: B});
        res.push(JPOS {pos: (else_block_instructions.len() as i64) + 2, adjust: false});
        res.extend(else_block_instructions.clone());
        res.push(JUMP {pos: (block_instructions.len() as i64) + 1, adjust: false});
        res.extend(block_instructions.clone());

        return res;
    }
pub fn handle_less_if(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(r, stack));
        res.push(SUB {pos: B});
        res.push(JPOS {pos: (else_block_instructions.len() as i64) + 2, adjust: false});
        res.extend(else_block_instructions.clone());
        res.push(JUMP {pos: (block_instructions.len() as i64) + 1, adjust: false});
        res.extend(block_instructions.clone());

        return res;
    }

pub fn handle_greaterequal_if(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(r, stack));
        res.push(SUB {pos: B});
        res.push(JPOS {pos: (block_instructions.len() as i64) + 2, adjust: false});
        res.extend(block_instructions.clone());
        res.push(JUMP {pos: (else_block_instructions.len() as i64) + 1, adjust: false});
        res.extend(else_block_instructions.clone());

        return res;
    }

pub fn handle_lessequal_if(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(r, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(l, stack));
        res.push(SUB {pos: B});
        res.push(JPOS {pos: (block_instructions.len() as i64) + 2, adjust: false});
        res.extend(block_instructions.clone());
        res.push(JUMP {pos: (else_block_instructions.len() as i64) + 1, adjust: false});
        res.extend(else_block_instructions.clone());

        return res;
    }
}
