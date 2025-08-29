/* compiler */

use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::{self, *};
use crate::ast::{self, *, Command::*};

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

        // firstly compile procedures i guess
        // then compile main and insert procedures 
        
        // declarations may be None
        // if let Some(command) = self.program.main.commands {
        //
        // }


        for command in self.program.main.commands {
            match command {
                Assign {name, expr} => println!("Assign"),
                If {cond, comm, else_comm} => println!("If"),
                While {cond, comm} => println!("While"),
                Repeat {comm, cond} => println!("Repeat"),
                Call {call} => println!("Call"),
                Read {name} => println!("Read"),
                Write {val} => println!("Write"),
            }
        }


        self.instructions.push(HALT);
        return self.instructions;
    }

    fn CommandWrite(val: i64) {
        
    }




}




