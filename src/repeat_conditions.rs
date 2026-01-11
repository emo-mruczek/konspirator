/* condition handling while */

// TODO: tests

use crate::ast::*;
use crate::compiler::Compiler;
use crate::errors::CompilerError;
use crate::helpers::Variable;
use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::HashMap;
use std::collections::HashSet;

impl Compiler {
    pub fn repeat_handle_equal(
        l: &Value,
        r: &Value,
        stack: &HashMap<String, Variable>,
        block_instructions: &Vec<Instruction>,
        initialized: &mut HashSet<String>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack, initialized)?);
        res.push(SWP { pos: B });
        res.extend(Self::handle_value(r, stack, initialized)?);
        res.push(SWP { pos: C });
        res.extend(Self::handle_value(r, stack, initialized)?);
        res.push(SUB { pos: B });

        let len = res.len();
        res.push(JPOS {
            pos: -((block_instructions.len() + len) as i64),
            adjust: true,
        });
        println!("{}", -((block_instructions.len() + len) as i64));

        res.push(SWP { pos: B });
        res.push(SUB { pos: C });

        res.push(JPOS {
            pos: -(((block_instructions.len() + len) as i64) + 3),
            adjust: true,
        });

        return Ok(res);
    }

    pub fn repeat_handle_notequal(
        l: &Value,
        r: &Value,
        stack: &HashMap<String, Variable>,
        block_instructions: &Vec<Instruction>,
        initialized: &mut HashSet<String>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack, initialized)?);
        res.push(SWP { pos: B });
        res.extend(Self::handle_value(r, stack, initialized)?);
        res.push(SWP { pos: C });
        res.extend(Self::handle_value(r, stack, initialized)?);
        res.push(SUB { pos: B });

        let len = res.len();

        res.push(JPOS {
            pos: 5,
            adjust: true,
        });

        res.push(SWP { pos: B });
        res.push(SUB { pos: C });
        res.push(JPOS {
            pos: 2,
            adjust: true,
        });

        res.push(JUMP {
            pos: -((block_instructions.len() + len) as i64),
            adjust: true,
        });

        return Ok(res);
    }

    pub fn repeat_handle_greater(
        l: &Value,
        r: &Value,
        stack: &HashMap<String, Variable>,
        block_instructions: &Vec<Instruction>,
        initialized: &mut HashSet<String>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(r, stack, initialized)?);
        res.push(SWP { pos: B });
        res.extend(Self::handle_value(l, stack, initialized)?);
        res.push(SUB { pos: B });

        res.push(JPOS {
            pos: 2,
            adjust: true,
        });

        let len = res.len();

        res.push(JUMP {
            pos: -((block_instructions.len() + len) as i64),
            adjust: true,
        });

        return Ok(res);
    }

    pub fn repeat_handle_less(
        l: &Value,
        r: &Value,
        stack: &HashMap<String, Variable>,
        block_instructions: &Vec<Instruction>,
        initialized: &mut HashSet<String>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack, initialized)?);
        res.push(SWP { pos: B });
        res.extend(Self::handle_value(r, stack, initialized)?);
        res.push(SUB { pos: B });

        res.push(JPOS {
            pos: 2,
            adjust: true,
        });

        let len = res.len();

        res.push(JUMP {
            pos: -((block_instructions.len() + len) as i64),
            adjust: true,
        });

        return Ok(res);
    }

    pub fn repeat_handle_greaterequal(
        l: &Value,
        r: &Value,
        stack: &HashMap<String, Variable>,
        block_instructions: &Vec<Instruction>,
        initialized: &mut HashSet<String>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack, initialized)?);
        res.push(SWP { pos: B });
        res.extend(Self::handle_value(r, stack, initialized)?);
        res.push(SUB { pos: B });

        let len = res.len();

        res.push(JPOS {
            pos: -((block_instructions.len() + len) as i64),
            adjust: true,
        });

        return Ok(res);
    }

    pub fn repeat_handle_lessequal(
        l: &Value,
        r: &Value,
        stack: &HashMap<String, Variable>,
        block_instructions: &Vec<Instruction>,
        initialized: &mut HashSet<String>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(r, stack, initialized)?);
        res.push(SWP { pos: B });
        res.extend(Self::handle_value(l, stack, initialized)?);
        res.push(SUB { pos: B });

        let len = res.len();

        res.push(JPOS {
            pos: -((block_instructions.len() + len) as i64),
            adjust: true,
        });

        return Ok(res);
    }
}
