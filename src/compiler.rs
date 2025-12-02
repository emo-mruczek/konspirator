/* compiler */


// TODO: liczby ujemne

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
    // TODO:
    // procedures: HashMap<String, ProcedureCompiler>,
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

        // TODO: procedures handling there before main
        
        match self.program.procedures {
            Some(procedures) => {
                for procedure in procedures {
                   // handle_procedure
                   // need for name change etc
                }
            }
            None => {
                println!("No procedures defined");
            }
        }

        // main
        // reserving the memory for the declared variables before main
        match self.program.main.declarations {
            Some(declarations) => {
                for variable in declarations {
                    match variable {
                        Declaration::Atomic {name} => {
                            self.stack.insert(name, Variable::Atomic {position: self.sp});

                            println!("SP: {}", self.sp);
                            self.sp += 1;
                        }
                        Declaration::Array {name, num_lhs, num_rhs} => {
                            self.stack.insert(name, Variable::Array {position: self.sp, lhs: num_lhs, rhs: num_rhs});
                            println!("SP: {}", self.sp);
                            self.sp += num_rhs - num_lhs; // prob ok
                        }
                    }
                }
                println!();
            }
            None => {
                println!("no variable declarations in Main");
            }
        }

        // compiling the main function

       self.instructions.extend(Self::handle_commands(&self.program.main.commands, &mut self.initialized, &self.stack));

        self.instructions.push(HALT);
        return self.instructions;
    }

    pub fn handle_commands(commands: &Vec<Command>, initialized: & mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut ret: Vec<Instruction> = vec![];

        for command in commands {
            match command {
                Assign {name, expr} => {
                    println!("  Assign");
                    let res = Self::command_assign(&name, &expr, initialized, &stack);
                    ret.extend(res);
                }
                If {cond, comm, else_comm} => {
                    println!("  If");
                    let res = Self::command_if(&cond, &comm, &else_comm, initialized, &stack);
                    ret.extend(res);
                },
                While {cond, comm} => {
                   println!("  While");
                   let res = Self::command_while(&cond, &comm, initialized, &stack);
                   ret.extend(res);
                },
                Repeat {comm, cond} => {
                   println!("  Repeat");
                   let res = Self::command_repeat(&cond, &comm, initialized, &stack);
                   ret.extend(res);
                },
            // TODO: change to cond? instead of vals 
                For {pid, val_lhs, val_rhs, comm, is_downto} => {
                    println!("  For");
                   // let res = Self::command_for(&cond, &comm, initialized, &stack, is_downto);
                    //ret.extend(res);
                }
                // TODO
                Call {call} => println!("Call"),
                Read {name} => {
                    println!("  Read");
                    let res = Self::command_read(&name, initialized, &stack);
                    ret.extend(res);
                }
                Write {val} => {
                    println!("  Write");
                    let res = Self::command_write(val, stack, initialized);
                    ret.extend(res);
                }
            }
        }

        return ret;
    }
}
