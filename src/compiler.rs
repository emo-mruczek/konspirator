/* compiler */

use crate::instructions::Instruction::{self, *};
use crate::ast::*;

// current instruction number can be obtained by checking the length od the instructions vector
pub struct Compiler {
    program: Program_All,
    instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new(program: Program_All) -> Self {
        Self {
            program: program,
            instructions: vec![],
        }
    }

    pub fn compile(mut self) -> Vec<Instruction> {
        self.instructions.push(HALT);
        return self.instructions;
    }
}


