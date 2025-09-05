/* compiler */

use crate::ast::{Command::*, Identifier::*, Expression::*, *};
use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::{HashMap, HashSet};

pub enum Variable {
    Atomic {position: u64},
    Array {position: u64, value: u64},
}

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

        /* zamienic na constructora  ze wzlegu na if */

        // replace with passing the actuall command to the command_ functions
        // for command in self.program.main.commands {
        //     match command {
        //         Assign {name, expr} => {
        //             let res = Self::command_assign(&name, &expr, &mut self.initialized, &self.stack);
        //             self.instructions.extend(res);
        //         }
        //         If {cond, comm, else_comm} => {
        //             let res = Self::command_if(&cond, &comm, &else_comm, &mut self.initialized, &self.stack);
        //
        //             self.instructions.extend(res);
        //         },
        //         While {cond, comm} => println!("While"),
        //         Repeat {comm, cond} => println!("Repeat"),
        //         Call {call} => println!("Call"),
        //         Read {name} => {
        //             let res = Self::command_read(&name, &mut self.initialized, &self.stack);
        //             self.instructions.extend(res);
        //         }
        //         Write {val} => {
        //             let res = Self::command_write(&val, &self.stack);
        //             self.instructions.extend(res);
        //         }
        //     }
        // }

        self.instructions.extend(Self::handle_commands(&self.program.main.commands, &mut self.initialized, &self.stack));

        self.instructions.push(HALT);
        return self.instructions;
    }


    fn handle_commands(commands: &Vec<Command>, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
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
                While {cond, comm} => println!("While"),
                Repeat {comm, cond} => println!("Repeat"),
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

    fn command_if(cond: &Condition, comm: &Vec<Command>, else_comm: &Option<Vec<Command>>, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        let mut block_instructions: Vec<Instruction> = vec![];
        let mut else_block_instructions: Vec<Instruction> = vec![];

        block_instructions.extend(Self::handle_commands(comm, initialized, stack));

        match else_comm {
            Some(commands) => else_block_instructions.extend(Self::handle_commands(commands, initialized, stack)),
            None => {},
        }

        match cond {
            Condition::Equal {l, r} => todo!(),
            Condition::NotEqual {l, r} => todo!(),
            Condition::Greater {l, r} => todo!(),
            Condition::Less {l, r} => todo!(),
            Condition::GreaterEqual {l, r} => todo!(),
            Condition::LessEqual {l, r} => todo!(),
        }
        
        return res;
    }

    /* condition handling for if */

    fn handle_equal_if(l: &Value, r: &Value, stack: &HashMap<String, Variable>, block_instructions: &Vec<Instruction>, else_block_instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(l, stack));
        res.push(PUT {pos: B});
        res.extend(Self::handle_value(r, stack));
        res.push(PUT {pos: C});
        res.push(SUB {pos: B});
        res.push(JPOS {pos: (block_instructions.len() as i64) + 5, adjust: false});
        res.push(GET {pos: B});
        res.push(SUB {pos: C});
        res.push(JPOS {pos: (block_instructions.len() as i64) + 2, adjust: false});
        res.extend(block_instructions.clone());
        res.push(JUMP {pos: (block_instructions.len() as i64) + 1, adjust: false});
        res.extend(else_block_instructions.clone());

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
        res.push(JZERO {pos: 14, adjust: true});
        res.push(SHR {pos: E});
        res.push(SHL {pos: E});
        res.push(GET {pos: C});
        res.push(SUB {pos: E});
        res.push(JZERO {pos: 4, adjust: true});
        res.push(GET {pos: D});
        res.push(ADD {pos: B});
        res.push(PUT {pos: D});
        res.push(SHL {pos: B});
        res.push(SHR {pos: C});
        res.push(GET {pos: C});
        res.push(PUT {pos: E});
        res.push(JUMP {pos: -14, adjust: true});
        res.push(GET {pos: D});

        return res;
    } 

    fn construct_division() -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.push(RST {pos: D});
        res.push(JZERO {pos: 21, adjust: true});
        res.push(GET {pos: C});
        res.push(SUB {pos: B});
        res.push(JPOS {pos: 18, adjust: true});
        res.push(GET {pos: C});
        res.push(PUT {pos: E});
        res.push(RST {pos: F});
        res.push(INC {pos: F});
        res.push(GET {pos: E});
        res.push(SUB {pos: B});
        res.push(JPOS {pos: 10, adjust: true});
        res.push(GET {pos: B});
        res.push(SUB {pos: E});
        res.push(PUT {pos: B});
        res.push(GET {pos: D});
        res.push(ADD {pos: F});
        res.push(PUT {pos: D});
        res.push(SHL {pos: E});
        res.push(SHL {pos: F});
        res.push(JUMP {pos: -11, adjust: true});
        res.push(JUMP {pos: -19, adjust: true});
        res.push(GET {pos: D});

        return res;
    } 

    fn construct_modulo() -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.push(RST {pos: D});
        res.push(JZERO {pos: 21, adjust: true});
        res.push(GET {pos: C});
        res.push(SUB {pos: B});
        res.push(JPOS {pos: 19, adjust: true});
        res.push(GET {pos: C});
        res.push(PUT {pos: E});
        res.push(RST {pos: F});
        res.push(INC {pos: F});
        res.push(GET {pos: E});
        res.push(SUB {pos: B});
        res.push(JPOS {pos: 10, adjust: true});
        res.push(GET {pos: B});
        res.push(SUB {pos: E});
        res.push(PUT {pos: B});
        res.push(GET {pos: D});
        res.push(ADD {pos: F});
        res.push(PUT {pos: D});
        res.push(SHL {pos: E});
        res.push(SHL {pos: F});
        res.push(JUMP {pos: -11, adjust: true});
        res.push(JUMP {pos: -19, adjust: true});
        res.push(RST {pos: B});
        res.push(GET {pos: B});

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
                res.extend(Self::get_variable(val, &stack, ));
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

    fn get_variable(id: &Identifier, stack: &HashMap<String, Variable>) -> Vec<Instruction> { // optional initialized
        let mut res: Vec<Instruction> = vec![];

        match id {
            Basic {name} => {
                let var = stack.get(name).unwrap(); // undeclared variable error todo
                res.extend(Self::handle_variable_atomic(var));
            }
            Array {name, size} => {
                let var = stack.get(name).unwrap(); // undeclared variable error todo
                res.extend(Self::handle_variable_array(var, *size));

            }
            VLA {name, size} => {
            //     if !initialized.contains(size) {
            //         panic!("not initialized"); //todo
            //     }
            //
            //     let ind_var = stack.get(size).unwrap(); // todo 
            //     res.extend(Self::handle_variable_atomic(var));                
            //
            //     res.push(LOAD {pos: A});
            //     res.push(PUT {pos: H});
            //
            //     let var = stack.get(name).unwrap(); //todo 
            //     match var {
            //         Variable::Atomic {position} => {
            //             println!("problemix");
            //         },
            //         Variable::Array {position, value} => {
            //             res.extend(Self::set_reg_a(*position));
            //         },
            //     }
            //
            //     res.push(ADD {pos: H});
             }
        }

        return res;
    }

    // change in orser to make it reasable from arrays in get_variable
    fn handle_variable_array(var: &Variable, size: u64) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        match var {
            Variable::Atomic {position} => {
                println!("problemix!"); // error todo
            },
            Variable::Array {position, value} => { // todo: maybe change the names
                if size >= *value {
                   println!("problemix! out od bounds"); // error out of bounds exception 
                }
                res.extend(Self::set_reg_a(position + size));
            },
        }

        return res;
    }

    fn handle_variable_atomic(var: &Variable) -> Vec<Instruction> {
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

    // todo: some improvements with shl

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
