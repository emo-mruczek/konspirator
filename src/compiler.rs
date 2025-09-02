/* compiler */

use crate::ast::{Command::*, Identifier::*, Value::*, *};
use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::{HashMap, HashSet};

pub enum Variable {
    Atomic {position: u64},
    // TODO:??
    Array {position: u64, value: u64},
}

// current instruction number can be obtained by checking the length od the instructions vector
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
                            print!(" basic ");
                            self.stack.insert(name, Variable::Atomic {position: self.sp});
                            self.sp += 1;
                        }
                        Declaration::Array {name, num} => {
                            // rename num to size later
                            print!(" array ");
                            self.stack.insert(name, Variable::Array {position: self.sp, value: num});
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

        for command in self.program.main.commands {
            match command {
                Assign {name, expr} => {
                    println!("Assign");
                    let res = Self::command_assign(&name, &expr, &mut self.initialized, &self.stack);
                    self.instructions.extend(res);
                }
                If {cond, comm, else_comm} => println!("If"),
                While {cond, comm} => println!("While"),
                Repeat {comm, cond} => println!("Repeat"),
                Call {call} => println!("Call"),
                Read {name} => {
                    print!("Read");
                    let res = Self::command_read(&name, &mut self.initialized, &self.stack);
                    self.instructions.extend(res);
                }
                Write {val} => {
                    print!("Write");
                    let res = Self::command_write(&val, &self.stack);
                    self.instructions.extend(res);
                }
            }
        }

        self.instructions.push(HALT);
        return self.instructions;
    }

    fn command_assign(id: &Identifier, expression: &Expression, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];
        let n: String;

        match id {
            Basic {name} => {
                println!(" basic");
                n = name.clone(); //TODO: ask

                let var = stack.get(&n).unwrap(); // undeclared variable error todoA

                res.extend(Self::handle_variable(var));
            }
            Array {name, size} => todo!(),
            VLA {name, size} => todo!(),
        }
        initialized.insert(n); // put it at the end of a scope

        res.push(PUT {pos: B});

        // check if initialized

        res.push(STORE {pos: B});
        return res;
    }

    fn command_read(id: &Identifier, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {

        let mut res: Vec<Instruction> = vec![];
        let mut n: String;

        match id {
            Basic { name } => {
                println!(" basic");
                n = name.clone(); //TODO: ask

                let var = stack.get(&n).unwrap(); // undeclared variable error todoA

                res.extend(Self::handle_variable(var));
            }
            Array {name, size} => {
                println!(" array");
                n = name.clone();
            }
            VLA {name, size} => {
                println!(" vla");
                n = name.clone();
            }
        }

        initialized.insert(n); // put it at the end of a scope

        // where is the variable stored?

        res.push(PUT {pos: B});
        res.push(READ);
        res.push(STORE {pos: B});

        return res;
    }

    // can be optimized but its irrevelant rn when there arent the exact instructions about this
    // years compilator
    // maybe something with into what register should i insert?
    fn command_write(val: &Value, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];
        // two cases
        // val is a i64
        // val is a var

        match val {
            Value::Num {val} => {
                println!(" num");
                let mut status: u64 = *val;
                res.push(RST {pos: A}); // A = 0
                if *val > 0 {
                    while status > 0 {
                        res.push(INC {pos: A});
                        status -= 1;
                    }
                }
            }
            Value::Var {val} => {
                println!(" var");
                let n: String;
                match val {
                    Basic { name } => {
                        println!(" basic");
                        n = name.clone(); //TODO: ask

                        let var = stack.get(&n).unwrap(); // undeclared variable error todoA

                        res.extend(Self::handle_variable(var));

                        res.push(LOAD {pos: A});
                    }
                    Array {name, size} => {
                        println!(" array");
                        n = name.clone();
                    }
                    VLA {name, size} => {
                        println!(" vla");
                        n = name.clone();
                    }
                }
            }
        }

        res.push(WRITE);
        return res;
    }

    /* helpers */

    fn handle_variable(var: &Variable) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        match var {
            Variable::Atomic {position} => {
                res.extend(Self::set_reg_a(*position));
            }
            Variable::Array {position, value} => {
                println!("problemix"); // error todo
            }
        }

        return res;
    }

    fn set_reg_a(position: u64) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];
        
        let mut status: u64 = position;
        res.push(RST {pos: A}); // A = 0
        if position > 0 {
            while status > 0 {
                res.push(INC {pos: A});
                status -= 1;
            }
        }

        return res;
    }
}
