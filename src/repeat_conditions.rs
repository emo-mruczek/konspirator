/* condition handling while */

use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::HashMap;
use crate::compiler::Compiler;
use crate::ast::*;
use crate::helpers::Variable;

impl Compiler {

    pub fn repeat_handle_equal(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(r, stack));
        res.push(PUT {pos: C});
        res.push(SUB {pos: B});

        let len = res.len();
        res.push(JPOS {pos: -((block_instructions.len() + len) as i64), adjust: true});
        println!("{}", -((block_instructions.len() + len) as i64));
        
        res.push(GET {pos: B});
        res.push(SUB {pos: C});

        res.push(JPOS {pos: -(((block_instructions.len() + len) as i64) + 3), adjust: true});

        return res;
    } 

    pub fn repeat_handle_notequal(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(r, stack));
        res.push(PUT {pos: C});
        res.push(SUB {pos: B});

        let len = res.len();

        res.push(JPOS {pos: 5, adjust: true});

        res.push(GET {pos: B});
        res.push(SUB {pos: C});
        res.push(JPOS {pos: 2, adjust: true});

        res.push(JUMP {pos: -((block_instructions.len() + len) as i64), adjust: true});

        return res;
    }
    
    pub fn repeat_handle_greater(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(r, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(l, stack));
        res.push(SUB {pos: B});

        res.push(JPOS {pos: 2, adjust: true});

        let len = res.len();

        res.push(JUMP {pos: -((block_instructions.len() + len) as i64), adjust: true});

        return res;
    }

    pub fn repeat_handle_less(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(r, stack));
        res.push(SUB {pos: B});

        res.push(JPOS {pos: 2, adjust: true});

        let len = res.len();

        res.push(JUMP {pos: -((block_instructions.len() + len) as i64), adjust: true});

        return res;
    }

    pub fn repeat_handle_greaterequal(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(r, stack));
        res.push(SUB {pos: B});

        let len = res.len();

        res.push(JPOS {pos: -((block_instructions.len() + len) as i64), adjust: true});

        return res;
    }

    pub fn repeat_handle_lessequal(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(r, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(l, stack));
        res.push(SUB {pos: B});

        let len = res.len();

        res.push(JPOS {pos: -((block_instructions.len() + len) as i64), adjust: true});

        return res;
    }
}
