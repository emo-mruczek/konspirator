/* compiler */

use crate::ast::{Command::*, *};
use crate::errors::CompilerError;
use crate::helpers::*;
use crate::instructions::Instruction::{self, *};
use crate::procedures_compiler::ProcedureCompiler;
use std::collections::{HashMap, HashSet};

pub struct Compiler {
    program: ProgramAll,
    instructions: Vec<Instruction>,
    stack: HashMap<String, Variable>, // its not a stack i know
    sp: u64,
    initialized: HashSet<String>,
    procedures: HashMap<String, ProcedureCompiler>,
}

impl Compiler {
    pub fn new(program: ProgramAll) -> Self {
        Self {
            program: program,
            instructions: vec![],
            stack: HashMap::new(),
            sp: 0,
            initialized: HashSet::new(),
            procedures: HashMap::new(),
        }
    }

    pub fn compile(mut self) -> Result<Vec<Instruction>, CompilerError> {
        match self.program.procedures {
            Some(procedures) => {
                for procedure in procedures {
                    self.procedures.insert(
                        procedure.proc_head.name.name.clone(),
                        ProcedureCompiler::new(procedure),
                    );
                    println!("   PROCEDURES    \n{:?}", self.procedures);
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
                        Declaration::Atomic { name } => {
                            self.stack
                                .insert(name.name, Variable::Atomic { position: self.sp });

                            println!("SP: {}", self.sp);
                            self.sp += 1;
                        }
                        Declaration::Array {
                            name,
                            num_lhs,
                            num_rhs,
                        } => {
                            self.stack.insert(
                                name.name,
                                Variable::Array {
                                    position: self.sp,
                                    lhs: num_lhs,
                                    rhs: num_rhs,
                                },
                            );
                            println!("SP: {}", self.sp);
                            self.sp += num_rhs - num_lhs + 1;
                        }
                    }
                }
            }
            None => {
                println!("no variable declarations in Main");
            }
        }

        // compiling the main function
        self.instructions.extend(Self::handle_commands(
            &self.program.main.commands,
            &mut self.initialized,
            &mut self.stack,
            &mut self.sp,
            &self.procedures,
        )?);

        self.instructions.push(HALT);

        return Ok(self.instructions);
    }

    pub fn handle_commands(
        commands: &Vec<Command>,
        initialized: &mut HashSet<String>,
        stack: &mut HashMap<String, Variable>,
        sp: &mut u64,
        procedures: &HashMap<String, ProcedureCompiler>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut ret: Vec<Instruction> = vec![];

        for command in commands {
            match command {
                Assign { name, expr } => {
                    println!("  Assign");
                    let res = Self::command_assign(&name, &expr, initialized, stack)?;
                    ret.extend(res);
                }
                If {
                    cond,
                    comm,
                    else_comm,
                } => {
                    println!("  If");
                    let res = Self::command_if(
                        &cond,
                        &comm,
                        &else_comm,
                        initialized,
                        stack,
                        sp,
                        procedures,
                    )?;
                    ret.extend(res);
                }
                While { cond, comm } => {
                    println!("  While");
                    let res =
                        Self::command_while(&cond, &comm, initialized, stack, sp, procedures)?;
                    ret.extend(res);
                }
                Repeat { comm, cond } => {
                    println!("  Repeat");
                    let res =
                        Self::command_repeat(&cond, &comm, initialized, stack, sp, procedures)?;
                    ret.extend(res);
                }
                For {
                    pid,
                    val_lhs,
                    val_rhs,
                    comm,
                    is_downto,
                } => {
                    println!("  For");
                    let res = Self::command_for(
                        &pid.name,
                        val_lhs,
                        val_rhs,
                        &comm,
                        *is_downto,
                        initialized,
                        stack,
                        sp,
                        procedures,
                    )?;
                    ret.extend(res);
                }
                Call { call } => {
                    println!(" Call");
                    let res = Self::command_call(call, procedures, initialized, stack, sp)?;
                    ret.extend(res);
                }
                Read { name } => {
                    println!("  Read");
                    let res = Self::command_read(&name, initialized, &stack)?;
                    ret.extend(res);
                }
                Write { val } => {
                    println!("  Write");
                    let res = Self::command_write(val, stack, initialized)?;
                    ret.extend(res);
                }
            }
        }

        return Ok(ret);
    }
}
