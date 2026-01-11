/* instructions */

use Instruction::*;
use Register::*;
use std::fmt;

#[derive(Debug, Clone)]
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
        let letter = format!("{:?}", self);
        match self {
            A | B | C | D | E | F | G | H => write!(f, "{}", letter.to_lowercase()),
        }
    }
}

// for easy printing instructions
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            READ | WRITE | HALT | RTRN => write!(f, "{:?}", self),
            LOAD { pos } => write!(f, "{} {}", "LOAD", pos),
            STORE { pos } => write!(f, "{} {}", "STORE", pos),
            RLOAD { pos } => write!(f, "{} {}", "RLOAD", pos),
            RSTORE { pos } => write!(f, "{} {}", "RSTORE", pos),
            ADD { pos } => write!(f, "{} {}", "ADD", pos),
            SUB { pos } => write!(f, "{} {}", "SUB", pos),
            SWP { pos } => write!(f, "{} {}", "SWP", pos),
            RST { pos } => write!(f, "{} {}", "RST", pos),
            INC { pos } => write!(f, "{} {}", "INC", pos),
            DEC { pos } => write!(f, "{} {}", "DEC", pos),
            SHL { pos } => write!(f, "{} {}", "SHL", pos),
            SHR { pos } => write!(f, "{} {}", "SHR", pos),
            JUMP { pos, adjust } => write!(f, "{} {}", "JUMP", pos),
            JPOS { pos, adjust } => write!(f, "{} {}", "JPOS", pos),
            JZERO { pos, adjust } => write!(f, "{} {}", "JZERO", pos),
            CALL { pos } => write!(f, "{} {}", "CALL", pos),
        }
    }
}

#[derive(Debug, Clone)] // in order to be able to print it
pub enum Instruction {
    READ,
    WRITE,
    LOAD { pos: i64 },
    STORE { pos: i64 },
    RLOAD { pos: Register },
    RSTORE { pos: Register },
    ADD { pos: Register },
    SUB { pos: Register },
    SWP { pos: Register },
    RST { pos: Register },
    INC { pos: Register },
    DEC { pos: Register },
    SHL { pos: Register },
    SHR { pos: Register },
    JUMP { pos: i64, adjust: bool },
    JPOS { pos: i64, adjust: bool },
    JZERO { pos: i64, adjust: bool },
    CALL { pos: Register },
    RTRN,
    HALT,
}
