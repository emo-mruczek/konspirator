mod ast;

use lalrpop_util::lalrpop_mod;
use std::{env, process::exit};

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
}
