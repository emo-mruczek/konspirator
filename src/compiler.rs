/* compiler */

use crate::ast::{Command::*, Identifier::*, Value::*, Expression::*, *};
use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::{HashMap, HashSet};

pub enum Variable {
    Atomic {position: u64},
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
                            self.stack.insert(name, Variable::Atomic {position: self.sp});
                            self.sp += 1;
                        }
                        Declaration::Array {name, num} => {
                            // rename num to size later
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
                    let res = Self::command_assign(&name, &expr, &mut self.initialized, &self.stack);
                    self.instructions.extend(res);
                }
                If {cond, comm, else_comm} => println!("If"),
                While {cond, comm} => println!("While"),
                Repeat {comm, cond} => println!("Repeat"),
                Call {call} => println!("Call"),
                Read {name} => {
                    let res = Self::command_read(&name, &mut self.initialized, &self.stack);
                    self.instructions.extend(res);
                }
                Write {val} => {
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

        res.extend(Self::get_variable(id, stack));

        res.push(PUT {pos: G});

        res.extend(Self::handle_expression(expression, initialized, stack));

        res.push(STORE {pos: G});
        
        initialized.insert(Self::get_name(id)); 

        return res;
    }

    fn command_read(id: &Identifier, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {

        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::get_variable(id, stack));

        res.push(PUT {pos: H});
        res.push(READ);
        res.push(STORE {pos: H});

        initialized.insert(Self::get_name(&id)); 

        return res;
    }

    fn command_write(val: &Value, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(val, stack));

        res.push(WRITE);
        return res;
    }

    /* expression handling */

    // check if initialized if expression has a variables

    fn handle_expression(expression: &Expression, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];
        
        match expression {
            Val {val} => {
                Self::is_initialized(&val, initialized);

                res.extend(Self::handle_value(val, stack));
            },
            Add {l, r} => {
                Self::is_initialized(&l, initialized);
                Self::is_initialized(&r, initialized);

                //TODO: perform addition in compile-time to reduce number of instructions
                
                res.extend(Self::handle_value(l, stack));
                res.push(PUT {pos: B});
                res.extend(Self::handle_value(r, stack));
                res.push(ADD {pos: B});
            },
            Sub {l, r} => {
                // obsluga ujemych???
                Self::is_initialized(&l, initialized);
                Self::is_initialized(&r, initialized);
                
                res.extend(Self::handle_value(r, stack));
                res.push(PUT {pos: B});
                res.extend(Self::handle_value(l, stack));
                res.push(SUB {pos: B});

            },
            Mul {l, r} => {
                Self::is_initialized(&l, initialized);
                Self::is_initialized(&r, initialized);
                
                res.extend(Self::handle_value(l, stack));
                res.push(PUT {pos: B});
                res.extend(Self::handle_value(r, stack));
                res.push(PUT {pos: C});
                res.extend(Self::construct_multiplication());

            },
            Div {l, r} => { //TODO: dzielenie przez zero
                Self::is_initialized(l, initialized);
                Self::is_initialized(r, initialized);

                res.extend(Self::handle_value(l, stack));
                res.push(PUT {pos: B});
                res.extend(Self::handle_value(r, stack));
                res.push(PUT {pos: C});
                res.extend(Self::construct_division());
            },
            Mod {l, r} => {
                Self::is_initialized(l, initialized);
                Self::is_initialized(r, initialized);

                res.extend(Self::handle_value(l, stack));
                res.push(PUT {pos: B});
                res.extend(Self::handle_value(r, stack));
                res.push(PUT {pos: C});
                res.extend(Self::construct_modulo());
            },
        }

        return res;
    }

    /* additional instructions */

    fn construct_multiplication() -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.push(PUT {pos: E});
        res.push(RST {pos: D});
        res.push(GET {pos: C});
        res.push(JZERO {pos: 14});
        res.push(SHR {pos: E});
        res.push(SHL {pos: E});
        res.push(GET {pos: C});
        res.push(SUB {pos: E});
        res.push(JZERO {pos: 4});
        res.push(GET {pos: D});
        res.push(ADD {pos: B});
        res.push(PUT {pos: D});
        res.push(SHL {pos: B});
        res.push(SHR {pos: C});
        res.push(GET {pos: C});
        res.push(PUT {pos: E});
        res.push(JUMP {pos: -14});
        res.push(GET {pos: D});

        return res;
    } 

    fn construct_division() -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.push(RST {pos: D});
        res.push(JZERO {pos: 21});
        res.push(GET {pos: C});
        res.push(SUB {pos: B});
        res.push(JPOS {pos: 18});
        res.push(GET {pos: C});
        res.push(PUT {pos: E});
        res.push(RST {pos: F});
        res.push(INC {pos: F});
        res.push(GET {pos: E});
        res.push(SUB {pos: B});
        res.push(JPOS {pos: 10});
        res.push(GET {pos: B});
        res.push(SUB {pos: E});
        res.push(PUT {pos: B});
        res.push(GET {pos: D});
        res.push(ADD {pos: F});
        res.push(PUT {pos: D});
        res.push(SHL {pos: E});
        res.push(SHL {pos: F});
        res.push(JUMP {pos: -11});
        res.push(JUMP {pos: -19});
        res.push(GET {pos: D});

        return res;
    } 

    fn construct_modulo() -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        return res;
    }

    /* helpers */

    fn handle_value(val: &Value, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        match val {
            Value::Num {val} => {
                res.extend(Self::set_reg_a(*val));
            },
            Value::Var {val} => {
                res.extend(Self::get_variable(val, &stack));
                res.push(LOAD {pos: A});
            },
        }

        return res;
    }

    fn is_initialized(val: &Value, initialized: &mut HashSet<String>) {
        match val {
            Value::Num {val} => {},
            Value::Var {val} => {
                if !initialized.contains(&Self::get_name(val)) {
                    panic!("not initialized"); // TODO: handling
                }
            }
        }
    }

    fn get_name(id: &Identifier) -> String {
        let name = match id {
            Basic {name} => name,
            Array {name, size} => name,
            VLA {name, size} => name,
        };

        return name.clone();
    }

    fn get_variable(id: &Identifier, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        match id {
            Basic {name} => {
                let var = stack.get(name).unwrap(); // undeclared variable error todoA
                res.extend(Self::handle_variable(var));
            }
            Array {name, size} => {
            }
            VLA {name, size} => {
            }
        }

        return res;
    }

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
