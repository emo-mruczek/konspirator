/* commands */

use crate::errors::CompilerError;
use crate::errors::CompilingErrorType::*;
use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::{HashMap, HashSet};
use crate::compiler::Compiler;
use crate::ast::*;
use crate::helpers::*;
use crate::ast::{Identifier::*, *};
use crate::procedures_compiler::ProcedureCompiler;


impl Compiler {


    pub fn command_assign(id: &Identifier, expression: &Expression, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) ->Result<Vec<Instruction>, CompilerError>  {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::get_variable(id, stack, initialized)?);

        res.push(SWP {pos: G}); // zamiana G z A

        res.extend(Self::handle_expression(expression, initialized, stack)?);

        res.push(RSTORE {pos: G}); // A = to co bylo w komorce odpowiadajacej temu co jest w get_variable
        
        initialized.insert(Self::get_name(id)); 

        return Ok(res);
    }

    pub fn command_read(id: &Identifier, initialized: &mut HashSet<String>, stack: &HashMap<String, Variable>) ->Result<Vec<Instruction>, CompilerError>  {

        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::get_variable(id, stack, initialized)?);

       // res.push(PUT {pos: G});
        res.push(SWP {pos: G});
        res.push(READ);
        res.push(RSTORE {pos: G});

        initialized.insert(Self::get_name(&id)); 

        return Ok(res);
    }



    pub fn command_write(val: &Value,  stack: &HashMap<String, Variable>, initialized: &mut HashSet<String>) ->Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(val, stack, initialized)?);
        
        res.push(WRITE); // wyswietl A

        return Ok(res);
    }

    pub fn handle_value(val: &Value,  stack: &HashMap<String, Variable>, initialized: &mut HashSet<String>) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        match val {
            Value::Num {val} => {
                res.extend(Self::set_reg_a(*val));
            },
            Value::Var {val} => {
                res.extend(Self::get_variable(val, &stack, initialized)?);
                res.push(RLOAD {pos: A}); // A = wartość w komórce o numerze będącym w A
            },
        }

        return Ok(res);
    }

    pub fn get_variable(id: &Identifier, stack: &HashMap<String, Variable>, initialized: &HashSet<String>) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        match id {
           Var {name} => {
                let variable = stack.get(&name.name);
                match variable {
                    Some(variable) => res.extend(Self::handle_variable_atomic(variable, name.clone())?),
                    None => return Err(CompilerError{error_type: UndeclaredVariable, id: name.name.clone(), pos: name.begin}),
                }
            }

            Array {name, var} => { // var is a num in this case
                let variable = stack.get(&name.name);
            match variable {
                    Some(variable) => res.extend(Self::handle_variable_array(variable, *var, name.clone())?),
                    None => return Err(CompilerError{error_type: UndeclaredVariable, id: name.name.clone(), pos: name.begin}),
                }
            }

           Array_Var {name, var} => {

                 let index_var_option = stack.get(&var.name);    
                let index_var;
                 match index_var_option {
                    Some(index_var_value) => index_var = index_var_value,
                    None => return Err(CompilerError{error_type: UndeclaredVariable, id: name.name.clone(), pos: name.begin}),
                }

                 if !initialized.contains(&var.name) {
                 return Err(CompilerError{error_type: VariableNotInitialized, id: var.name.clone(), pos: var.begin});
                 }
                
                 let array_var = stack.get(&name.name);
                 match array_var {
                    Some(array_var) => {
                    match array_var {
                    Variable::Atomic {position} => {

                        return Err(CompilerError{error_type: IncorrectUseOfVariable, id: name.name.clone()  , pos: name.begin });
                     },
                     Variable::Array {position, lhs, rhs} => {
                        
                        res.extend(Self::set_reg_a(*lhs)); // A = lhs wartosc 
                        res.push(SWP {pos: F}); // F = A, czyli lhs wartosc 
                        match index_var {
                            Variable::Array {position, rhs, lhs} => {
                                return Err(CompilerError{error_type: IncorrectUseOfVariable, id: var.name.clone(), pos: *position as usize });
                            }
                            Variable::Atomic {position} => {

                            res.push(LOAD {pos: *position as i64});
                            }
                        }
                        res.push(SUB {pos: F}); // A = F - A, czyyyli w A mamy offset!
                        res.push(SWP {pos: F}); // w F offset 
                        res.extend(Self::set_reg_a(*position));
                        res.push(ADD {pos: F});
                     },
                 }


                    },
                    None => return Err(CompilerError{error_type: UndeclaredVariable, id: name.name.clone(), pos: name.begin}),
                }
            
                           
            }
        }

        return Ok(res);
    } 


    pub fn handle_variable_array(var: &Variable, value: u64, name: PID) -> Result<Vec<Instruction>, CompilerError> { 

        // array but num-indexed
        let mut res: Vec<Instruction> = vec![];

        match var {
            Variable::Atomic {position} => {
            return Err(CompilerError{error_type: IncorrectUseOfVariable, id: name.name, pos: *position as usize});
            },
            Variable::Array {position, lhs, rhs} => {
                if value > *rhs || value < *lhs {
                return Err(CompilerError{error_type: IndexOutOfBounds, id: name.name, pos: *position as usize });
                }
                let offset: u64 = value - lhs; 
                res.extend(Self::set_reg_a(position + offset));
            },
        }

        return Ok(res);
    }
    
    pub fn handle_variable_atomic(var: &Variable, name: PID) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        match var {
            Variable::Atomic {position} => {
                res.extend(Self::set_reg_a(*position));
            }
            Variable::Array {position, lhs, rhs} => {
                return Err(CompilerError{error_type: IncorrectUseOfVariable, id: name.name, pos: *position as usize} );
            }
        }

        return Ok(res);
    }

     pub fn command_if(cond: &Condition, comm: &Vec<Command>, else_comm: &Option<Vec<Command>>, initialized: &mut HashSet<String>, stack: &mut HashMap<String, Variable>, sp: u64, procedures: &HashMap<String, ProcedureCompiler>) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        let mut block_instructions: Vec<Instruction> = vec![];
        let mut else_block_instructions: Vec<Instruction> = vec![];

        block_instructions.extend(Self::handle_commands(comm, initialized, stack, sp, procedures)?);

        match else_comm {
            Some(commands) => else_block_instructions.extend(Self::handle_commands(commands, initialized, stack, sp, procedures)?),
            None => {},
        }

        match cond {
            Condition::Equal {l, r} => {
                res.extend(Self::if_handle_equal(l, r, stack, &block_instructions, &else_block_instructions, initialized)?);
            },
            Condition::NotEqual {l, r} => {
                res.extend(Self::if_handle_notequal(l, r, stack, &block_instructions, &else_block_instructions, initialized)?);
            },
            Condition::Greater {l, r} => {
                res.extend(Self::if_handle_greater(l, r, stack, &block_instructions, &else_block_instructions, initialized)?);
            },
            Condition::Less {l, r} => {
                res.extend(Self::if_handle_less(l, r, stack, &block_instructions, &else_block_instructions, initialized)?);
            },
            Condition::GreaterEqual {l, r} => {
                res.extend(Self::if_handle_greaterequal(l, r, stack, &block_instructions, &else_block_instructions, initialized)?);
            },
            Condition::LessEqual {l, r} => {
                res.extend(Self::if_handle_lessequal(l, r, stack, &block_instructions, &else_block_instructions, initialized)?);
            },
        }
        
        return Ok(res);
    }

        pub fn command_while(cond: &Condition, comm: &Vec<Command>, initialized: &mut HashSet<String>, stack: &mut HashMap<String, Variable>, sp: u64, procedures: &HashMap<String, ProcedureCompiler> ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        let mut block_instructions: Vec<Instruction> = vec![];
        block_instructions.extend(Self::handle_commands(comm, initialized, stack, sp, procedures)?);

        match cond {
            Condition::Equal {l, r} => {
                res.extend(Self::while_handle_equal(l, r, stack, &block_instructions, initialized)?);
            },
            Condition::NotEqual {l, r} => {
                res.extend(Self::while_handle_notequal(l, r, stack, &block_instructions, initialized)?);
            },
            Condition::Greater {l, r} => {
                res.extend(Self::while_handle_greater(l, r, stack, &block_instructions, initialized)?);
            },
            Condition::Less {l, r} => {
                res.extend(Self::while_handle_less(l, r, stack, &block_instructions, initialized)?);
            },
            Condition::GreaterEqual {l, r} => {
                res.extend(Self::while_handle_greaterequal(l, r, stack, &block_instructions, initialized)?);
            },
            Condition::LessEqual {l, r} => {
                res.extend(Self::while_handle_lessequal(l, r, stack, &block_instructions, initialized)?);
            },
        }

        res.extend(block_instructions);
        res.push(JUMP {pos: -(res.len() as i64), adjust: true});

        return Ok(res);
    }
    
    pub fn command_repeat(cond: &Condition, comm: &Vec<Command>, initialized: &mut HashSet<String>, stack: &mut HashMap<String, Variable>, sp: u64, procedures: &HashMap<String, ProcedureCompiler>) -> Result<Vec<Instruction>, CompilerError>  {
        let mut res: Vec<Instruction> = vec![];
        let mut conditions: Vec<Instruction> = vec![];

        let mut block_instructions: Vec<Instruction> = vec![];
        block_instructions.extend(Self::handle_commands(comm, initialized, stack, sp, procedures)?);

        match cond {
            Condition::Equal {l, r} => {
                conditions.extend(Self::repeat_handle_equal(l, r, stack, &block_instructions, initialized)?);
            },
            Condition::NotEqual {l, r} => {
                conditions.extend(Self::repeat_handle_notequal(l, r, stack, &block_instructions, initialized)?);
            },
            Condition::Greater {l, r} => {
                conditions.extend(Self::repeat_handle_greater(l, r, stack, &block_instructions, initialized)?);
            },
            Condition::Less {l, r} => {
                conditions.extend(Self::repeat_handle_less(l, r, stack, &block_instructions, initialized)?);
            },
            Condition::GreaterEqual {l, r} => {
                conditions.extend(Self::repeat_handle_greaterequal(l, r, stack, &block_instructions, initialized)?);
            },
            Condition::LessEqual {l, r} => {
                conditions.extend(Self::repeat_handle_lessequal(l, r, stack, &block_instructions, initialized)?);
            },
        }

        res.extend(block_instructions);
        res.extend(conditions);

        return Ok(res);
    }

    // TODO: check na wartosci
     pub fn command_for(pid: &String, val_lhs: &Value, val_rhs: &Value,  comm: &Vec<Command>, is_downto: bool, initialized: &mut HashSet<String>, stack: &mut HashMap<String, Variable>, mut sp: u64, procedures: &HashMap<String, ProcedureCompiler>) -> Result<Vec<Instruction>, CompilerError>  {
         let mut res: Vec<Instruction> = vec![];

        /* musimy jakos zainicjalizowac zmienna */

        // TODO: error ze nie moze to byc array
        stack.insert(pid.clone(), Variable::Atomic {position: sp});
        sp += 1;

        /* wartosc i to lhs */
        /* troche jak w assign czy cos */
        /* wiec czemu by nie zrobic assign? */

        //res.push(SWP {pos: G}); // zamiana G z A to chyba ne potrzebne w ogole?

        //res.extend(Self::handle_expression(expression, initialized, stack));
        // obslugujemy tylko jeden case wiec: 
        res.extend(Self::handle_value(val_lhs, stack, initialized)?);

        // res.push(RSTORE {pos: G}); // A = to co bylo w komorce odpowiadajacej temu co jest w get_variable
        
        initialized.insert(pid.clone()); 

        // TODO: lldb sprawdzic, czy sie ta zmoienna ustawia i ile wynosi czy cos

        // w tym miejscu, zmienna, po ktorej iterujemy, powinna juz byc zainicjalizowana

        /* FOR zmienna FROM wrtosc TO/DOWNTO wartosc DO commands ENDFOR */

        let mut block_instructions: Vec<Instruction> = vec![];
        block_instructions.extend(Self::handle_commands(comm, initialized, stack, sp, procedures)?); // mamy juz
        // wrzucone commands, wartoscia obecna zmiennej, po ktorej iterujemy, zajmuje sie maszyna
        // wirtualna; my musimy zapewni jumpa odpowiedniego, oraz zmniejszanie zmiennej bądź
        // zwiekszanie zmiennej o jeden 
        // jump kiedy odejmowanie jumppos czy cos

        // IDEA: dwie sciezki w zaleznosci, czy jest ustawiony is_downto, moze jeden wielki if xd
        if (is_downto) {
  // tak naprawde to jest dopoki, doputy lhs > rhs albo lhs >= rhs, no a przy tym up na
            // odwrot
            // wiec to troche taki while???

            res.extend(Self::handle_value(val_lhs, stack, initialized)?); // zamiast lhs wyciagamy
            // zmienna, po ktorej iterujemy
            res.push(SWP {pos: B});
            res.extend(Self::handle_value(val_rhs, stack, initialized)?);
            res.push(SUB {pos: B});
            res.push(JPOS {pos: (block_instructions.len() as i64) + 2, adjust: true});

            /* odjac jeden od wartosci zmiennej */

        } else {
                      
            

        }

        res.extend(block_instructions);

        /* clearing the stack */
        stack.remove(pid);
        sp -= 1; 
        initialized.remove(pid);

         return Ok(res);
    }


 pub fn command_call(call: &ProcCall,  procedures: &HashMap<String, ProcedureCompiler>,initialized: &mut HashSet<String>,  stack: &mut HashMap<String, Variable>, mut sp: u64) -> Result<Vec<Instruction>, CompilerError>  {

    let mut res: Vec<Instruction> = vec![];

    // TODO: recursive_procedure_call error check there 
        // check czy procedura istnieje
        // check na liczbe argumentow
        // ogólnie checki 

//         let procedure_name: String = call.name.name.clone();
//         let procedure_compiler: &ProcedureCompiler = procedures.get(&procedure_name).unwrap();
//         let procedure_declarations: Option<Declarations> = procedure_compiler.get_declarations();
//         let call_arguments: Args = call.args.clone();
//         let procedure_arguments: ArgsDecl = procedure_compiler.get_declared_arguments();
//
//         match procedure_declarations {
//             Some(&declarations) => {
//                 for variable in declarations {
//                     match variable {
//                            Declaration::Atomic {name} => {
//                             stack.insert(format!("{}@{}", name.name, procedure_name), Variable::Atomic {position: sp});
//
//                             println!("SP: {}", sp);
//                             sp += 1;
//                         }
//                         Declaration::Array {name, num_lhs, num_rhs} => {
//                             stack.insert(format!("{}@{}", name.name, procedure_name), Variable::Array {position: sp, lhs: num_lhs, rhs: num_rhs});
//                             println!("SP: {}", sp);
//                             sp += num_rhs - num_lhs + 1;
//                         }
//                     }
//                 }
//             },
//             None => println!("Nothing declared!"),
//         }
//
//         // iterujemy po argumentach wywołania oraz argumentach z procedury
//         for (argument, declared_argument) in call_arguments.iter().zip(procedure_arguments) {
//
// let variable_id: PID;
//                         let argument_id: PID;
//
//             match procedure_declarations {
//                 Some(declarations) => {
//                     for variable in declarations {
//
//
//
//                         match variable {
//                             Declaration::Atomic { name } => variable_id = name,
//                             Declaration::Array { name, num_lhs, num_rhs } => variable_id = name,
//                         }
//
//                         argument_id = declared_argument.name;
//                         // there some check maybe for I O ?
//                         // match declared_argument.type_name {
//                         //     Type::Array => argument_id ,
//                         //     Type::Const => todo!(),
//                         //     Type::Undefined => todo!(),
//                         //     Type::Scalar => todo!(),
//                         // }
//
//                         // TODO: duplicate variable declaration error
//
//
//                     }
//                 },
//                 None => println!("No procedure declaration"),
//             }
//
//
//             // TODO: this unwrap
//             let pointer: &Variable = stack.get(&argument.name).unwrap();
//
//             let declared_argument_name: String = declared_argument.name.name;
//             match declared_argument.type_name {
//                 Type::Array => {
//                     match pointer {
//                         Variable::Atomic { position } => {
//  // TODO: error!,
//                         println!("problemix!");
//
//                         },
//                         Variable::Array { position, lhs, rhs } => {
//                          initialized.insert(argument.name);
//                         stack.insert(format!("{}@{}", variable_id.name, procedure_name), Variable::Array { position: *position, lhs: *lhs, rhs: *rhs });
//                             initialized.insert(format!("{}@{}", variable_id.name, procedure_name));
//                         }
//                     }
//                 },
//
//                 Type::Const => todo!(),
//                 Type::Undefined => todo!(),
//                 Type::Scalar => todo!(),
//             }
//
//
//         }
//
//
//        res.extend(Self::handle_commands(&procedure_compiler.get_commands(), initialized, stack, sp, procedures)?);
//
//
    return Ok(res);
}
}




