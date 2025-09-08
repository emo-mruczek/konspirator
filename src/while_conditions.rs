/* condition handling while if */

use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::HashMap;
use crate::compiler::Compiler;
use crate::ast::*;
use crate::helpers::Variable;

impl Compiler {

    pub fn while_handle_equal(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(r, stack));
        res.push(PUT {pos: C});
        res.push(SUB {pos: B});
        res.push(JPOS {pos: (block_instructions.len() as i64) + 5, adjust: true});
        res.push(GET {pos: B});
        res.push(SUB {pos: C});
        res.push(JPOS {pos: (block_instructions.len() as i64) + 2, adjust: true});

        res.extend(block_instructions.clone());
        res.push(JUMP {pos:(else_block_instructions.len() as i64) + 1, adjust: true});

        res.extend(else_block_instructions.clone());

        return res;
    } 

    pub fn while_handle_notequal(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(r, stack));
        res.push(PUT {pos: C});
        res.push(SUB {pos: B});
        res.push(JPOS {pos: (else_block_instructions.len() as i64) + 5, adjust: true});
        res.push(GET {pos: B});
        res.push(SUB {pos: C});
        res.push(JPOS {pos: (else_block_instructions.len() as i64) + 2, adjust: true});

        res.extend(else_block_instructions.clone());

        res.push(JUMP {pos: (block_instructions.len() as i64) + 1, adjust: true});
        res.extend(block_instructions.clone());

        return res;
    }
    
    pub fn while_handle_greater(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(r, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(l, stack));
        res.push(SUB {pos: B});
        res.push(JPOS {pos: (else_block_instructions.len() as i64) + 2, adjust: true});

        res.extend(else_block_instructions.clone());

        res.push(JUMP {pos: (block_instructions.len() as i64) + 1, adjust: true});
        res.extend(block_instructions.clone());

        return res;
    }

    pub fn while_handle_less(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(r, stack));
        res.push(SUB {pos: B});
        res.push(JPOS {pos: (else_block_instructions.len() as i64) + 2, adjust: true});

        res.extend(else_block_instructions.clone());

        res.push(JUMP {pos: (block_instructions.len() as i64) + 1, adjust: true});
        res.extend(block_instructions.clone());

        return res;
    }

    pub fn while_handle_greaterequal(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(r, stack));
        res.push(SUB {pos: B});
        res.push(JPOS {pos: (block_instructions.len() as i64) + 2, adjust: true});
        res.extend(block_instructions.clone());
        res.push(JUMP {pos: (else_block_instructions.len() as i64) + 1, adjust: true});

        res.extend(else_block_instructions.clone());

        return res;
    }

    pub fn while_handle_lessequal(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(r, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(l, stack));
        res.push(SUB {pos: B});
        res.push(JPOS {pos: (block_instructions.len() as i64) + 2, adjust: true});
        res.extend(block_instructions.clone());
        res.push(JUMP {pos: (else_block_instructions.len() as i64) + 1, adjust: true});

        res.extend(else_block_instructions.clone());

        return res;
    }
}
