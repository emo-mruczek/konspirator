/* commands */

use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::{HashMap, HashSet};
use crate::compiler::Compiler;
use crate::ast::*;
use crate::helpers::*;

impl Compiler {


// poddałom się ideologii &self 
// chyba musiałom do tego dojrzeć
  //  pub fn command_assign(&mut self) -> Vec<Instruction> {

    //}

    pub fn command_write(val: &Value) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(val));
        
        res.push(WRITE); // wyswietl A

        return res;
    }

    pub fn handle_value(val: &Value) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        match val {
            Value::Num {val} => {
                res.extend(Self::set_reg_a(*val));
            },
            Value::Var {val} => {
              //  res.extend(Self::get_variable(val, &stack, ));
              //  res.push(LOAD {pos: A});
            },
        }

        return res;
    }

   }
