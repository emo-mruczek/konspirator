mod ast;
mod commands;
mod compiler;
mod errors;
mod expressions;
mod helpers;
mod if_conditions;
mod instructions;
mod procedures_compiler;
mod repeat_conditions;
mod while_conditions;

use crate::{
    compiler::Compiler,
    errors::CompilerError,
    errors::CompilingErrorType::*,
    instructions::Instruction::{self, *},
};
use lalrpop_util::lalrpop_mod;
use std::io::BufReader;
use std::{
    env,
    fs::{self, File},
    io::{self, BufRead, Write},
    process::exit,
};

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

    let input_code =
        fs::read_to_string(in_name).expect("\x1b[31mCouldn't read the input file\x1b[0m");

    /* actual compilation */

    println!("\n Compiling:\n\n {}", input_code);

    /* parsing */

    let program = parser::PROGRAMALLParser::new().parse(&input_code);

    println!(" Successfully parsed\n");
    println!("{:#?}\n", program);

    let mut instructions: Vec<Instruction> = vec![];

    match program {
        Ok(p) => {
            let compiler: Compiler = Compiler::new(p);

            let instructions_result: Result<Vec<Instruction>, CompilerError> = compiler.compile();

            match instructions_result {
                Ok(result) => instructions = result,
                Err(error) => handle_error(error, in_name),
            }
        }
        Err(e) => panic!("Syntax error: {e:}"),
    };

    /* needed to recheck the instructions in order to set the jump positions */

    /* output */

    println!("\n Compiled code:\n");

    for (iter, instruction) in instructions.iter_mut().enumerate() {
        match instruction {
            JUMP { pos, adjust } | JPOS { pos, adjust } | JZERO { pos, adjust } => {
                if *adjust {
                    *pos = (iter as i64) + *pos;
                    while *pos < 0 {
                        *pos = (iter as i64) + *pos;
                    }
                }
            }
            _ => {}
        }
        println!("{}", instruction);
    }

    let mut output_file = File::create(out_name).expect("Error while creating a file");

    for instruction in instructions.iter() {
        write!(output_file, "{}\n", instruction).expect("Error while writting a file");
    }

    return Ok(());
}

fn handle_error(error: CompilerError, input: &String) {
    let line = find_line(input, error.pos).expect("Unable to get position");

    match error.error_type {
        UndeclaredVariable => {
            let id: String = error
                .id
                .split('@')
                .next()
                .expect("Error while splitting")
                .to_string();
            panic!("Error: UndeclaredVariable {} on line {}", id, line);
        }
        MultipleVariableDeclarations => {
            let id: String = error
                .id
                .split('@')
                .next()
                .expect("Error while splitting")
                .to_string();
            panic!(
                "Error: MultipleVariableDeclarations {} on line {}",
                id, line
            );
        }
        UndeclaredProcedure => {
            panic!("Error: UndeclaredProcedure {} on line {}", error.id, line);
        }
        MultipleProcedureDeclaration => {
            panic!(
                "Error: MultipleProcedureDeclaration {} on line {}",
                error.id, line
            );
        }
        RecursiveProcedureCall => {
            panic!(
                "Error: RecursiveProcedureCall {} on line {}",
                error.id, line
            );
        }
        IndexOutOfBounds => {
            let id: String = error
                .id
                .split('@')
                .next()
                .expect("Error while splitting")
                .to_string();
            panic!("Error: IndexOutOfBounds {} on line {}", id, line);
        }
        ArrayTypeVariableAsIndex => {
            let id: String = error
                .id
                .split('@')
                .next()
                .expect("Error while splitting")
                .to_string();
            panic!("Error: ArrayTypeVariableAsIndex {} on line {}", id, line);
        }
        IncorrectTypeArgument => {
            let id: String = error
                .id
                .split('@')
                .next()
                .expect("Error while splitting")
                .to_string();
            panic!("Error: ArrayTypeVariableAsIndex {} on line {}", id, line);
        }
        IncorrectNumberOfArguments => {
            panic!(
                "Error: RecursiveProcedureCall {} on line {}",
                error.id, line
            );
        }
        IncorrectUseOfVariable => {
            let id: String = error
                .id
                .split('@')
                .next()
                .expect("Error while splitting")
                .to_string();
            panic!("Error: IncorrectUseOfVariable {} on line {}", id, line);
        }
        VariableNotInitialized => {
            let id: String = error
                .id
                .split('@')
                .next()
                .expect("Error while splitting")
                .to_string();
            panic!("Error: VariableNotInitialized {} on line {}", id, line);
        }
    };
}

// TODO:
fn find_line(input: &String, bytes: usize) -> Option<usize> {
    let file = File::open(input).expect("Error while opening a file");
    let reader = BufReader::new(file);

    let mut total_bytes = 0;
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        total_bytes += line.len() + 1; // +1 for the '\n' character
        if total_bytes >= bytes {
            return Some(i + 1); // +1 because line numbers start from 1
        }
    }

    return None; // return None if bytes is greater than the total number of bytes
}
