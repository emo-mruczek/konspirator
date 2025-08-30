/* compiler */

use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::{self, *};
use crate::ast::{*, Command::*, Value::*,};

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
                Write {val} => {
                    print!("Write");
                    let res = Self::command_write(&val);
                    self.instructions.extend(res);
                    },
            }
        }


        self.instructions.push(HALT);
        return self.instructions;
    }

    // can be optimized but its irrevelant rn when there arent the exact instructions about this
    // years compilator
    // maybe something with into what register should i insert?
    fn command_write(val: &Value) -> Vec<Instruction> {
        let mut res = vec![];
        // two cases
        // val is a i64
        // val is a var
        
        match val {
            Value::Num {val} => { 
                println!(" num");
                let mut status: i64 = *val;
                res.push(RST {pos: A}); // A = 0
                if *val > 0 {
                    while status > 0 {
                        res.push(INC {pos: A});
                        status -= 1;
                    }
                } else if *val < 0 {
                    while status < 0 {
                        res.push(DEC {pos: A});
                        status += 1;
                    }
                }
                res.push(WRITE);               
            }, 
            Var {val} => println!("var"),
        }

        return res; 
    }
}




