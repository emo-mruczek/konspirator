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
                let var = stack.get(name).unwrap(); // undeclared variable error todo
                //res.extend(Self::handle_variable_atomic(var));
            }
            Array {name, var} => {
                let var = stack.get(name).unwrap(); // undeclared variable error todo
                //res.extend(Self::handle_variable_array(var, *size));

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
}
