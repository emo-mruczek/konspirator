/* compiler */

use crate::ast::{Command::*, *};
use crate::instructions::Instruction::{self, *};
use std::collections::{HashMap, HashSet};
use crate::helpers::*;

pub struct Compiler {
    program: ProgramAll,
    instructions: Vec<Instruction>,
    stack: HashMap<String, Variable>, // its not a stack i know
    sp: u64,
    initialized: HashSet<String>,
}

impl Compiler {
    pub fn new(program: ProgramAll) -> Self {
        Self {
            program: program,
            instructions: vec![],
            stack: HashMap::new(),
            sp: 0,
            initialized: HashSet::new(),
        }
    }

    pub fn compile(mut self) -> Vec<Instruction> {

        // main
        match self.program.main.declarations {
            Some(declarations) => {
                for variable in declarations {
                    match variable {
                        Declaration::Basic {name} => {
                            // rename basic to atomic later
                            self.stack.insert(name, Variable::Atomic {position: self.sp});

                            println!("SP: {}", self.sp);
                            self.sp += 1;
                        }
                        Declaration::Array {name, num} => {
                            // rename num to size later
                            self.stack.insert(name, Variable::Array {position: self.sp, value: num});
                            println!("SP: {}", self.sp);
                            self.sp += num;
                        }
                    }
                }
                println!();
            }
            None => {
                println!("no variable declarations in Main");
            }
        }

        self.instructions.extend(Self::handle_commands(&self.program.main.commands, &mut self.initialized, &self.stack));

        self.instructions.push(HALT);
        return self.instructions;
    }


    pub fn handle_commands(commands: &Vec<Command>, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut ret: Vec<Instruction> = vec![];

        for command in commands {
            match command {
                Assign {name, expr} => {
                    let res = Self::command_assign(&name, &expr, initialized, &stack);
                    ret.extend(res);
                }
                If {cond, comm, else_comm} => {
                    let res = Self::command_if(&cond, &comm, &else_comm, initialized, &stack);

                    ret.extend(res);
                },
                While {cond, comm} => {
                    let res = Self::command_while(&cond, &comm, initialized, &stack);

                    ret.extend(res);
 
                },
                Repeat {comm, cond} => {
                    let res = Self::command_repeat(&cond, &comm, initialized, &stack);

                    ret.extend(res);
 
                },
                Call {call} => println!("Call"),
                Read {name} => {
                    let res = Self::command_read(&name, initialized, &stack);
                    ret.extend(res);
                }
                Write {val} => {
                    let res = Self::command_write(&val, &stack);
                    ret.extend(res);
                }
            }
        }

        return ret;
    }
    
      }
