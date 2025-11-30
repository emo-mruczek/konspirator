/* commands */

use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::{HashMap, HashSet};
use crate::compiler::Compiler;
use crate::ast::*;
use crate::helpers::*;
use crate::ast::{Identifier::*, *};

impl Compiler {


    pub fn command_assign(id: &Identifier, expression: &Expression, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::get_variable(id, stack, initialized));

       // res.push(PUT {pos: G}); // bylo: G = A
        res.push(SWP {pos: G}); // zamiana G z A

        res.extend(Self::handle_expression(expression, initialized, stack));

        res.push(RSTORE {pos: G}); // A = to co bylo w komorce odpowiadajacej temu co jest w get_variable
        // TODO: moze swap lepszy??????????? bo szybszy
        //res.push(SWP {pos: G});
        
        initialized.insert(Self::get_name(id)); 

        return res;
    }



    pub fn command_write(val: &Value,  stack: &HashMap<String, Variable>, initialized: &HashSet<String>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(val, stack, initialized));
        
        res.push(WRITE); // wyswietl A

        return res;
    }

    pub fn handle_value(val: &Value,  stack: &HashMap<String, Variable>, initialized: &HashSet<String>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        match val {
            Value::Num {val} => {
                res.extend(Self::set_reg_a(*val));
            },
            Value::Var {val} => {
                res.extend(Self::get_variable(val, &stack, initialized));
                res.push(RLOAD {pos: A}); // A = wartość w komórce o numerze będącym w A
            },
        }

        return res;
    }

    pub fn get_variable(id: &Identifier, stack: &HashMap<String, Variable>, initialized: &HashSet<String>) -> Vec<Instruction> { // optional initialized
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
           // TODO: !!!!!!!!
            // no i przenieść to do swojej własnej funkcji
            Array_Var {name, var} => {
                 if !initialized.contains(var) {
                     panic!("not initialized"); // TODOL error returning
                 }

                // TODO; out of bounds exeption
            
                 let index_var = stack.get(var).unwrap(); // TODO: error returning
                 res.extend(Self::handle_variable_atomic(index_var));  

                 let array_var = stack.get(name).unwrap();
            
                 res.push(RLOAD {pos: A});
                 //res.push(PUT {pos: H});
                 res.push(SWP {pos: H}); // H = A
            
                match array_var {
                    Variable::Atomic {position} => {
                         println!("problemix");
                     },
                     Variable::Array {position, lhs, rhs} => {
  
                         // TODO: bounds checking
                        // sprawdzenie, ile wynosi wartosc variable 
                        // sprawdzenie, czy ta wartosc jest w bounds 
                        // pozycja w regex obliczona jak w Array indeksowanym wtedy wartością
                        // zwykłą (liczbą)
                         res.extend(Self::set_reg_a(*position));
                     },
                 }
            
                res.push(ADD {pos: H});
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
