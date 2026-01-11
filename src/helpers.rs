/* errors DONE */
/* helper functions */


use crate::ast::{Identifier::*, *};
use crate::errors::{CompilerError, CompilingErrorType::{self, *}};
use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::{HashMap, HashSet};

use crate::compiler::Compiler;

pub enum Variable {
    Atomic {position: u64},
    Array {position: u64, lhs: u64, rhs: u64},
}

impl Compiler {


    // OK
     pub fn set_reg_a(position: u64) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![]; 

        res.push(RST {pos: A}); // A = 0
    
        if position == 0 {
            return res;
        }

        let mut status: u64 = position;
        let mut value_construction: Vec<Instruction> = vec![];

        while status > 0 {
            if status % 2 == 1 {
                    value_construction.push(INC {pos: A});
                    status -= 1;
            } else {
                     value_construction.push(SHL {pos: A});
                     status /= 2;
            }
        }

        value_construction.reverse();
        res.extend(value_construction);

        return res;
    }

    pub fn is_initialized(val: &Value, initialized: &mut HashSet<String>) -> Result<bool, CompilerError> {
        match val {
            Value::Num {val} => {},
            Value::Var {val} => {
                let var_name = &Self::get_name(val);
                if !initialized.contains(var_name) {
                    return Err(CompilerError{error_type: VariableNotInitialized, id: var_name.clone(), pos: 0 }); // TODO: ?
                }
            }
        }

        return Ok(true);
    }

    pub fn get_name(id: &Identifier) -> String {
        let name = match id {
            Var {name} => name,
            Array {name, var} => name,
            Array_Var {name, var} => name,
        };

        return name.name.clone();
    }

}
