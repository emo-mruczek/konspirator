/* helper functions */

use crate::ast::{Identifier::*, *};
use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::{HashMap, HashSet};

use crate::compiler::Compiler;

pub enum Variable {
    Atomic {position: u64},
    Array {position: u64, value: u64},
}
impl Compiler {

    pub fn handle_value(val: &Value, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
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

    pub fn is_initialized(val: &Value, initialized: &mut HashSet<String>) {
        match val {
            Value::Num {val} => {},
            Value::Var {val} => {
                if !initialized.contains(&Self::get_name(val)) {
                    panic!("not initialized"); // TODO: handling
                }
            }
        }
    }

    pub fn get_name(id: &Identifier) -> String {
        let name = match id {
            Basic {name} => name,
            Array {name, size} => name,
            VLA {name, size} => name,
        };

        return name.clone();
    }

    pub fn get_variable(id: &Identifier, stack: &HashMap<String, Variable>) -> Vec<Instruction> { // optional initialized
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
    pub fn handle_variable_array(var: &Variable, size: u64) -> Vec<Instruction> {
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

    pub fn handle_variable_atomic(var: &Variable) -> Vec<Instruction> {
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

    pub fn set_reg_a(position: u64) -> Vec<Instruction> {
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
