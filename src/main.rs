// TODO: next -> naming the main params

mod ast;

use lalrpop_util::lalrpop_mod;
use std::{env, io::{self, Write}, fs::{self, File}, process::exit};


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

   

    /* parsing */

    let program = parser::PROGRAMALLParser::new().parse(&input_code);

    match program {
        Ok(_) => { //_ for now, in order to not move the program
            println!(" Successfully parsed\n");
        },
        Err(e) => panic!("Something's wrong! {e:}"), // TODO: for now
    };


        
    // whole ast dump
    println!("{:#?}", program); 
    
    // https://stackoverflow.com/questions/63713887/how-to-write-string-to-file
    let mut output_code = File::create(out_name)?;

    return Ok(());
}


