/* expression handling */

use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::{HashMap, HashSet};
use crate::compiler::Compiler;
use crate::ast::{Expression::*, *};
use crate::helpers::Variable;

impl Compiler {
    
    pub fn handle_expression(expression: &Expression, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];
        
        match expression {
            Val {val} => {
                Self::is_initialized(&val, initialized);

                res.extend(Self::handle_value(val, stack, initialized));
            },
            Add {l, r} => {
                 Self::is_initialized(&l, initialized);
                 Self::is_initialized(&r, initialized);
                
                 //TODO: perform addition in compile-time to reduce number of instructions
                
                 res.extend(Self::handle_value(l, stack, initialized));
                 res.push(SWP {pos: B});
                 res.extend(Self::handle_value(r, stack, initialized));
                 res.push(ADD {pos: B});
            },
            Sub {l, r} => {
                // obsluga ujemych???
                Self::is_initialized(&l, initialized);
                Self::is_initialized(&r, initialized);

                res.extend(Self::handle_value(r, stack, initialized));
                res.push(SWP {pos: B});
                res.extend(Self::handle_value(l, stack, initialized));
                res.push(SUB {pos: B});

            },
            Mul {l, r} => {
// TODO
                Self::is_initialized(&l, initialized);
                Self::is_initialized(&r, initialized);

                res.extend(Self::handle_value(l, stack, initialized));
                res.push(SWP {pos: B});
                res.extend(Self::handle_value(r, stack, initialized));
                res.push(SWP {pos: C});
                res.extend(Self::construct_multiplication());

            },
            Div {l, r} => { //TODO: dzielenie przez zero
                // TODO:
                Self::is_initialized(l, initialized);
                Self::is_initialized(r, initialized);

                res.extend(Self::handle_value(l, stack, initialized));
                res.push(SWP {pos: B});
                res.extend(Self::handle_value(r, stack, initialized));
                res.push(SWP {pos: C});
                res.extend(Self::construct_division());
            },
            Mod {l, r} => {
                // TODO
                Self::is_initialized(l, initialized);
                Self::is_initialized(r, initialized);

                res.extend(Self::handle_value(l, stack, initialized));
                res.push(SWP {pos: B});
                res.extend(Self::handle_value(r, stack, initialized));
                res.push(SWP {pos: C});
                res.extend(Self::construct_modulo());
            },
        }

        return res;
    }

// TODOL:!!!!!!
    pub fn construct_multiplication() -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];
        // w B i C są już mnożone wartości    

       // res.push(PUT {pos: E}); // do E kladziemy A 
        res.push(SWP {pos: E});  // E == A i A == E (zamiana)
        res.push(RST {pos: D}); // D = 0
        
       // res.push(GET {pos: C}); // A == C
        res.push(SWP {pos:C});

        res.push(JZERO {pos: 14, adjust: true});
        res.push(SHR {pos: E});
        res.push(SHL {pos: E});

       // res.push(GET {pos: C});
        res.push(SWP {pos:C});

        res.push(SUB {pos: E});
        res.push(JZERO {pos: 4, adjust: true});


       // res.push(GET {pos: D});
        res.push(SWP {pos: D});

        res.push(ADD {pos: B});

        // res.push(PUT {pos: D});
        res.push(SWP{pos: D});

        res.push(SHL {pos: B});
        res.push(SHR {pos: C});


       // res.push(GET {pos: C});
          res.push(SWP {pos:C});

       // res.push(PUT {pos: E});
          res.push(SWP {pos: E});

        res.push(JUMP {pos: -14, adjust: true});


       // res.push(GET {pos: D});
        res.push(SWP {pos:D});

        return res;
    } 

    pub fn construct_division() -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];
        //
        // res.push(RST {pos: D});
        // res.push(JZERO {pos: 21, adjust: true});
        // res.push(GET {pos: C});
        // res.push(SUB {pos: B});
        // res.push(JPOS {pos: 18, adjust: true});
        // res.push(GET {pos: C});
        // res.push(PUT {pos: E});
        // res.push(RST {pos: F});
        // res.push(INC {pos: F});
        // res.push(GET {pos: E});
        // res.push(SUB {pos: B});
        // res.push(JPOS {pos: 10, adjust: true});
        // res.push(GET {pos: B});
        // res.push(SUB {pos: E});
        // res.push(PUT {pos: B});
        // res.push(GET {pos: D});
        // res.push(ADD {pos: F});
        // res.push(PUT {pos: D});
        // res.push(SHL {pos: E});
        // res.push(SHL {pos: F});
        // res.push(JUMP {pos: -11, adjust: true});
        // res.push(JUMP {pos: -19, adjust: true});
        // res.push(GET {pos: D});

        return res;
    } 

    pub fn construct_modulo() -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        // res.push(RST {pos: D});
        // res.push(JZERO {pos: 21, adjust: true});
        // res.push(GET {pos: C});
        // res.push(SUB {pos: B});
        // res.push(JPOS {pos: 19, adjust: true});
        // res.push(GET {pos: C});
        // res.push(PUT {pos: E});
        // res.push(RST {pos: F});
        // res.push(INC {pos: F});
        // res.push(GET {pos: E});
        // res.push(SUB {pos: B});
        // res.push(JPOS {pos: 10, adjust: true});
        // res.push(GET {pos: B});
        // res.push(SUB {pos: E});
        // res.push(PUT {pos: B});
        // res.push(GET {pos: D});
        // res.push(ADD {pos: F});
        // res.push(PUT {pos: D});
        // res.push(SHL {pos: E});
        // res.push(SHL {pos: F});
        // res.push(JUMP {pos: -11, adjust: true});
        // res.push(JUMP {pos: -19, adjust: true});
        // res.push(RST {pos: B});
        // res.push(GET {pos: B});

        return res;
    }
}
