/* condition handling for if */

/* adjust should be true??????? */

/* TO TESTS!!!!!! */

use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::HashMap;
use crate::compiler::Compiler;
use crate::ast::*;
use crate::helpers::Variable;

impl Compiler {

    pub fn handle_equal(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Option<Vec<Instruction>>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        let else_len = match else_block_instructions {
            Some(instructions) => instructions.len() as i64,
            None => 0,
        };

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
        res.push(JUMP {pos: else_len + 1, adjust: true});

        match else_block_instructions {
            Some(instructions) => res.extend(instructions.clone()),
            None => {},
        }

        return res;
    } 

    pub fn handle_notequal(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Option<Vec<Instruction>>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        let else_len = match else_block_instructions {
            Some(instructions) => instructions.len() as i64,
            None => 0,
        };

        res.extend(Self::handle_value(l, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(r, stack));
        res.push(PUT {pos: C});
        res.push(SUB {pos: B});
        res.push(JPOS {pos: else_len + 5, adjust: true});
        res.push(GET {pos: B});
        res.push(SUB {pos: C});
        res.push(JPOS {pos: else_len + 2, adjust: true});

        match else_block_instructions {
            Some(instructions) => res.extend(instructions.clone()),
            None => {},
        }

        res.push(JUMP {pos: (block_instructions.len() as i64) + 1, adjust: true});
        res.extend(block_instructions.clone());

        return res;
    }
    
    pub fn handle_greater(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Option<Vec<Instruction>>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        let else_len = match else_block_instructions {
            Some(instructions) => instructions.len() as i64,
            None => 0,
        };

        res.extend(Self::handle_value(r, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(l, stack));
        res.push(SUB {pos: B});
        res.push(JPOS {pos: else_len + 2, adjust: true});

        match else_block_instructions {
            Some(instructions) => res.extend(instructions.clone()),
            None => {},
        }

        res.push(JUMP {pos: (block_instructions.len() as i64) + 1, adjust: true});
        res.extend(block_instructions.clone());

        return res;
    }

    pub fn handle_less(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Option<Vec<Instruction>>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        let else_len = match else_block_instructions {
            Some(instructions) => instructions.len() as i64,
            None => 0,
        };

        res.extend(Self::handle_value(l, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(r, stack));
        res.push(SUB {pos: B});
        res.push(JPOS {pos: else_len + 2, adjust: true});

        match else_block_instructions {
            Some(instructions) => res.extend(instructions.clone()),
            None => {},
        }

        res.push(JUMP {pos: (block_instructions.len() as i64) + 1, adjust: true});
        res.extend(block_instructions.clone());

        return res;
    }

    pub fn handle_greaterequal(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Option<Vec<Instruction>>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        let else_len = match else_block_instructions {
            Some(instructions) => instructions.len() as i64,
            None => 0,
        };

        res.extend(Self::handle_value(l, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(r, stack));
        res.push(SUB {pos: B});
        res.push(JPOS {pos: (block_instructions.len() as i64) + 2, adjust: true});
        res.extend(block_instructions.clone());
        res.push(JUMP {pos: else_len + 1, adjust: true});

        match else_block_instructions {
            Some(instructions) => res.extend(instructions.clone()),
            None => {},
        }

        return res;
    }

    pub fn handle_lessequal(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Option<Vec<Instruction>>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        let else_len = match else_block_instructions {
            Some(instructions) => instructions.len() as i64,
            None => 0,
        };

        res.extend(Self::handle_value(r, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(l, stack));
        res.push(SUB {pos: B});
        res.push(JPOS {pos: (block_instructions.len() as i64) + 2, adjust: true});
        res.extend(block_instructions.clone());
        res.push(JUMP {pos: else_len + 1, adjust: true});

        match else_block_instructions {
            Some(instructions) => res.extend(instructions.clone()),
            None => {},
        }

        return res;
    }
}
