/* helper functions */

use crate::ast::{Identifier::*, *};
use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::{HashMap, HashSet};

use crate::compiler::Compiler;

pub enum Variable {
    Atomic {position: u64},
    Array {position: u64, lhs: u64, rhs: u64},
}

impl Compiler {


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

}
