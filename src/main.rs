mod ast;

use lalrpop_util::lalrpop_mod;
use std::{env, fmt, fs, process::exit};
use crate::Instruction::*;

lalrpop_mod!(parser);

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("\x1b[31mProvide 2 arguments!\x1b[0m");
        exit(1);
    }

    let in_name = &args[1];
    let out_name = &args[2];

    println!(" Input file: {}\n Output file {}", in_name, out_name);

    let input_code = fs::read_to_string(in_name).expect("\x1b[31mCouldn't read the input file\x1b[0m");

    println!("\n Compiling:\n\n {}", input_code);

    let mut instructions: Vec<Instruction> = vec![]; 

    parser::PROGRAM_ALLParser::new().parse(&input_code).unwrap();

    // compilu compilu

    instructions.push(LOAD{pos: 1});
    instructions.push(HALT);

    // printu printu compiled code

    println!(" Compiled code:\n");

    for instruction in instructions.iter() {
        println!("{}", instruction); 
    }
}

// progams ends with HALT
// vector with instructions to write as an output

// for easy printing instructions
impl fmt::Display for Instruction {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // probably shoud not leave _ for instructions with position due to possible bugs, but
            // idc
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
enum Instruction {
    READ,
    WRITE,
    LOAD {
        pos: u64, 
    },
    STORE {
        pos: u64, 
    },
    ADD {
        pos: u64, 
    },
    SUB {
        pos: u64, 
    },
    GET {
        pos: u64, 
    },
    PUT {
        pos: u64, 
    },
    RST {
        pos: u64, 
    },
    INC {
        pos: u64, 
    },
    DEC {
        pos: u64, 
    },
    SHL {
        pos: u64, 
    },
    SHR {
        pos: u64, 
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
        pos: u64, 
    },
    JUMPR {
        pos: u64, 
    },
    HALT,
}

