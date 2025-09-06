use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use crate::compiler::Compiler;

impl Compiler {
    
    pub fn construct_multiplication() -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.push(PUT {pos: E});
        res.push(RST {pos: D});
        res.push(GET {pos: C});
        res.push(JZERO {pos: 14, adjust: true});
        res.push(SHR {pos: E});
        res.push(SHL {pos: E});
        res.push(GET {pos: C});
        res.push(SUB {pos: E});
        res.push(JZERO {pos: 4, adjust: true});
        res.push(GET {pos: D});
        res.push(ADD {pos: B});
        res.push(PUT {pos: D});
        res.push(SHL {pos: B});
        res.push(SHR {pos: C});
        res.push(GET {pos: C});
        res.push(PUT {pos: E});
        res.push(JUMP {pos: -14, adjust: true});
        res.push(GET {pos: D});

        return res;
    } 

    pub fn construct_division() -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.push(RST {pos: D});
        res.push(JZERO {pos: 21, adjust: true});
        res.push(GET {pos: C});
        res.push(SUB {pos: B});
        res.push(JPOS {pos: 18, adjust: true});
        res.push(GET {pos: C});
        res.push(PUT {pos: E});
        res.push(RST {pos: F});
        res.push(INC {pos: F});
        res.push(GET {pos: E});
        res.push(SUB {pos: B});
        res.push(JPOS {pos: 10, adjust: true});
        res.push(GET {pos: B});
        res.push(SUB {pos: E});
        res.push(PUT {pos: B});
        res.push(GET {pos: D});
        res.push(ADD {pos: F});
        res.push(PUT {pos: D});
        res.push(SHL {pos: E});
        res.push(SHL {pos: F});
        res.push(JUMP {pos: -11, adjust: true});
        res.push(JUMP {pos: -19, adjust: true});
        res.push(GET {pos: D});

        return res;
    } 

    pub fn construct_modulo() -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.push(RST {pos: D});
        res.push(JZERO {pos: 21, adjust: true});
        res.push(GET {pos: C});
        res.push(SUB {pos: B});
        res.push(JPOS {pos: 19, adjust: true});
        res.push(GET {pos: C});
        res.push(PUT {pos: E});
        res.push(RST {pos: F});
        res.push(INC {pos: F});
        res.push(GET {pos: E});
        res.push(SUB {pos: B});
        res.push(JPOS {pos: 10, adjust: true});
        res.push(GET {pos: B});
        res.push(SUB {pos: E});
        res.push(PUT {pos: B});
        res.push(GET {pos: D});
        res.push(ADD {pos: F});
        res.push(PUT {pos: D});
        res.push(SHL {pos: E});
        res.push(SHL {pos: F});
        res.push(JUMP {pos: -11, adjust: true});
        res.push(JUMP {pos: -19, adjust: true});
        res.push(RST {pos: B});
        res.push(GET {pos: B});

        return res;
    }
}
