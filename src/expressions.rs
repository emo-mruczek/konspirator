/* expression handling */

use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::{HashMap, HashSet};
use crate::compiler::Compiler;
use crate::ast::{Expression::*, *};
use crate::helpers::Variable;

/* todo: move there stuff from additional_commands, and this function into compiler.rs */

impl Compiler {
    
    pub fn handle_expression(expression: &Expression, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
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



}
