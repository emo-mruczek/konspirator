mod ast;

use lalrpop_util::lalrpop_mod;
use std::{env, fs, process::exit};

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

    instructions.push(Instruction::HALT);

    // printu printu compiled code

    println!(" Compiled code:\n");

    for instruction in instructions.iter() {
        println!("{:?}", instruction); 
    }
}

// progams ends with HALT
// vector with instructions to write as an output



// add strings?
#[derive(Debug)] // in order to be able to print it
enum Instruction {
    READ,
    WRITE,
    LOAD,
    STORE,
    ADD,
    SUB,
    GET,
    PUT,
    RST,
    INC,
    DEC,
    SHL,
    SHR,
    JUMP,
    JPOS,
    JZERO,
    STRK,
    JUMPR,
    HALT,
}

