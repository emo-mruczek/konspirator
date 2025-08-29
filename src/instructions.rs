/* instructions */

use std::fmt;
use Instruction::*;
use Register::*;

#[derive(Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl fmt::Display for Register {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            A | B | C | D | E | F | G | H => write!(f, "{:?}", self),
        }
    }
}

// for easy printing instructions
impl fmt::Display for Instruction {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            READ | WRITE | HALT => write!(f, "{:?}", self),
            LOAD {pos} => write!(f, "{} {}", "LOAD", pos),
            STORE {pos} => write!(f, "{} {}", "STORE", pos),
            ADD {pos} => write!(f, "{} {}", "ADD", pos),
            SUB {pos} => write!(f, "{} {}", "SUB", pos),
            GET {pos} => write!(f, "{} {}", "GET", pos),
            PUT {pos} => write!(f, "{} {}", "PUT", pos),
            RST {pos} => write!(f, "{} {}", "RST", pos),
            INC {pos} => write!(f, "{} {}", "INC", pos),
            DEC {pos} => write!(f, "{} {}", "DEC", pos),
            SHL {pos} => write!(f, "{} {}", "SHL", pos),
            SHR {pos} => write!(f, "{} {}", "SHR", pos),
            JUMP {pos} => write!(f, "{} {}", "JUMP", pos),
            JPOS {pos} => write!(f, "{} {}", "JPOS", pos),
            JZERO {pos} => write!(f, "{} {}", "JZERO", pos),
            JUMPR {pos} => write!(f, "{} {}", "JUMPR", pos),
            STRK {pos} => write!(f, "{} {}", "STRK", pos),
        }
    }
}

#[derive(Debug)] // in order to be able to print it
pub enum Instruction {
    READ,
    WRITE,
    LOAD {
        pos: Register, 
    },
    STORE {
        pos: Register, 
    },
    ADD {
        pos: Register, 
    },
    SUB {
        pos: Register, 
    },
    GET {
        pos: Register, 
    },
    PUT {
        pos: Register, 
    },
    RST {
        pos: Register, 
    },
    INC {
        pos: Register, 
    },
    DEC {
        pos: Register, 
    },
    SHL {
        pos: Register, 
    },
    SHR {
        pos: Register, 
    },
    JUMP {
        pos: u64, 
    },
    JPOS {
        pos: u64, 
    },
    JZERO {
        pos: u64, 
    },
    STRK {
        pos: Register, 
    },
    JUMPR {
        pos: Register, 
    },
    HALT,
}

