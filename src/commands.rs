/* commands */

use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::{HashMap, HashSet};
use crate::compiler::Compiler;
use crate::ast::*;
use crate::helpers::*;
use crate::ast::{Identifier::*, *};

impl Compiler {


  //  pub fn command_assign(&mut self) -> Vec<Instruction> {

    //}

    pub fn command_write(val: &Value,  stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(val, stack));
        
        res.push(WRITE); // wyswietl A

        return res;
    }

    pub fn handle_value(val: &Value,  stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        match val {
            Value::Num {val} => {
                res.extend(Self::set_reg_a(*val));
            },
            Value::Var {val} => {
                res.extend(Self::get_variable(val, &stack));
                res.push(RLOAD {pos: A}); // A = wartość w komórce o numerze będącym w A
            },
        }

        return res;
    }

    pub fn get_variable(id: &Identifier, stack: &HashMap<String, Variable>) -> Vec<Instruction> { // optional initialized
        let mut res: Vec<Instruction> = vec![];

        match id {
            Var {name} => {
                let variable = stack.get(name).unwrap(); // undeclared variable error todo
                res.extend(Self::handle_variable_atomic(variable));
            }
            Array {name, var} => { // var is a num in this case
                let variable = stack.get(name).unwrap(); // undeclared variable error todo
                res.extend(Self::handle_variable_array(variable, *var));

            }
            Array_Var {name, var} => {
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

    // array but variable-indexed
    pub fn handle_variable_array_variable(var: &Variable, value: u64) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];




        return res;
    }

    pub fn handle_variable_array(var: &Variable, value: u64) -> Vec<Instruction> { // array but
        // num-indexed
        let mut res: Vec<Instruction> = vec![];

        match var {
            Variable::Atomic {position} => {
                println!("problemix!"); // error todo
            },
            Variable::Array {position, lhs, rhs} => {
                // TODO::
                if value >= *rhs || value < *lhs {
                   println!("problemix! out od bounds"); // error out of bounds exception 
                }
                let offset: u64 = value - lhs; 
                res.extend(Self::set_reg_a(position + offset));
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
            Variable::Array {position, lhs, rhs} => {
                println!("Error"); // TODO;
            }
        }

        return res;
    }
}
