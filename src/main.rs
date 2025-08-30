// TODO: get rid of warnings

mod ast;
mod instructions;
mod compiler;

use lalrpop_util::lalrpop_mod;
use std::{env, io::{self, Write}, fs::{self, File}, process::exit};
use crate::{compiler::Compiler, instructions::Instruction::{self, *}};
lalrpop_mod!(parser);

fn main() -> io::Result<()> {

    /* input */

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("\x1b[31mProvide 2 arguments!\x1b[0m");
        exit(1);
    }

    let in_name = &args[1];
    let out_name = &args[2];

    println!(" Input file: {}\n Output file {}", in_name, out_name);

    let input_code = fs::read_to_string(in_name).expect("\x1b[31mCouldn't read the input file\x1b[0m");

    /* actual compilation */

    println!("\n Compiling:\n\n {}", input_code);

    let program = parser::PROGRAM_ALLParser::new().parse(&input_code);

    let mut instructions: Vec<Instruction> = vec![]; 
    match program {
        Ok(p) => {
            println!(" Successfully parsed\n");
            let compiler: Compiler = Compiler::new(p);
            instructions = compiler.compile();
        },
        Err(_) => panic!("Something wrong!"), // TODO: for now
    };

    /* output */

    println!("\n Compiled code:\n");

    for instruction in instructions.iter() {
        println!("{}", instruction); 
    }

    // https://stackoverflow.com/questions/63713887/how-to-write-string-to-file
    let mut output_code = File::create(out_name)?;
    
    for instruction in instructions.iter() {
        write!(output_code, "{}\n", instruction)?;
    }

    return Ok(());
}


