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

    pub fn command_read(id: &Identifier, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {

        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::get_variable(id, stack, initialized));

       // res.push(PUT {pos: G});
        res.push(SWP {pos: G});
        res.push(READ);
        res.push(RSTORE {pos: G});

        initialized.insert(Self::get_name(&id)); 

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
                 //res.extend(Self::handle_variable_atomic(index_var)); 
                
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
                        // okay wiec set reg a musi byc dosc specjalnie obsluzony - byc moze nowy
                        // dla tego wlasnie przypadku? nie wiemu, ile wynosi w naszym przypadku
                        // offset, poniewaz kryje się on za zmienną, tak więc nie mam pojecia
                         let offset: u64 = 0;
                         res.extend(Self::set_reg_a(position + offset));
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

     pub fn command_if(cond: &Condition, comm: &Vec<Command>, else_comm: &Option<Vec<Command>>, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        let mut block_instructions: Vec<Instruction> = vec![];
        let mut else_block_instructions: Vec<Instruction> = vec![];

        block_instructions.extend(Self::handle_commands(comm, initialized, stack));

        match else_comm {
            Some(commands) => else_block_instructions.extend(Self::handle_commands(commands, initialized, stack)),
            None => {},
        }

        match cond {
            Condition::Equal {l, r} => {
                res.extend(Self::if_handle_equal(l, r, stack, &block_instructions, &else_block_instructions));
            },
            Condition::NotEqual {l, r} => {
                res.extend(Self::if_handle_notequal(l, r, stack, &block_instructions, &else_block_instructions));
            },
            Condition::Greater {l, r} => {
                res.extend(Self::if_handle_greater(l, r, stack, &block_instructions, &else_block_instructions));
            },
            Condition::Less {l, r} => {
                res.extend(Self::if_handle_less(l, r, stack, &block_instructions, &else_block_instructions));
            },
            Condition::GreaterEqual {l, r} => {
                res.extend(Self::if_handle_greaterequal(l, r, stack, &block_instructions, &else_block_instructions));
            },
            Condition::LessEqual {l, r} => {
                res.extend(Self::if_handle_lessequal(l, r, stack, &block_instructions, &else_block_instructions));
            },
        }
        
        return res;
    }

        pub fn command_while(cond: &Condition, comm: &Vec<Command>, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        let mut block_instructions: Vec<Instruction> = vec![];
        block_instructions.extend(Self::handle_commands(comm, initialized, stack));

        match cond {
            Condition::Equal {l, r} => {
                res.extend(Self::while_handle_equal(l, r, stack, &block_instructions));
            },
            Condition::NotEqual {l, r} => {
                res.extend(Self::while_handle_notequal(l, r, stack, &block_instructions));
            },
            Condition::Greater {l, r} => {
                res.extend(Self::while_handle_greater(l, r, stack, &block_instructions));
            },
            Condition::Less {l, r} => {
                res.extend(Self::while_handle_less(l, r, stack, &block_instructions));
            },
            Condition::GreaterEqual {l, r} => {
                res.extend(Self::while_handle_greaterequal(l, r, stack, &block_instructions));
            },
            Condition::LessEqual {l, r} => {
                res.extend(Self::while_handle_lessequal(l, r, stack, &block_instructions));
            },
        }

        res.extend(block_instructions);
        res.push(JUMP {pos: -(res.len() as i64), adjust: true});

        return res;
    }
    
    pub fn command_repeat(cond: &Condition, comm: &Vec<Command>, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];
        let mut conditions: Vec<Instruction> = vec![];

        let mut block_instructions: Vec<Instruction> = vec![];
        block_instructions.extend(Self::handle_commands(comm, initialized, stack));

        match cond {
            Condition::Equal {l, r} => {
                conditions.extend(Self::repeat_handle_equal(l, r, stack, &block_instructions));
            },
            Condition::NotEqual {l, r} => {
                conditions.extend(Self::repeat_handle_notequal(l, r, stack, &block_instructions));
            },
            Condition::Greater {l, r} => {
                conditions.extend(Self::repeat_handle_greater(l, r, stack, &block_instructions));
            },
            Condition::Less {l, r} => {
                conditions.extend(Self::repeat_handle_less(l, r, stack, &block_instructions));
            },
            Condition::GreaterEqual {l, r} => {
                conditions.extend(Self::repeat_handle_greaterequal(l, r, stack, &block_instructions));
            },
            Condition::LessEqual {l, r} => {
                conditions.extend(Self::repeat_handle_lessequal(l, r, stack, &block_instructions));
            },
        }

        res.extend(block_instructions);
        res.extend(conditions);

        return res;
    }

}
