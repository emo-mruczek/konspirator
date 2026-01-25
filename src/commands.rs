/* commands */

use crate::ast::Command::*;
use crate::ast::*;
use crate::ast::{Identifier::*, *};
use crate::compiler::Compiler;
use crate::errors::CompilingErrorType::*;
use crate::errors::{CompilerError, CompilingErrorType};
use crate::helpers::*;
use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use crate::procedures_compiler::ProcedureCompiler;
use std::collections::{HashMap, HashSet};

impl Compiler {
    pub fn command_assign(
        id: &Identifier,
        expression: &Expression,
        initialized: &mut HashSet<String>,
        stack: &HashMap<String, Variable>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::get_variable(id, stack, initialized)?);

        res.push(SWP { pos: G }); // zamiana G z A

        res.extend(Self::handle_expression(expression, initialized, stack)?);

        res.push(RSTORE { pos: G }); // A = to co bylo w komorce odpowiadajacej temu co jest w get_variable

        initialized.insert(Self::get_name(id));

        return Ok(res);
    }

    pub fn command_read(
        id: &Identifier,
        initialized: &mut HashSet<String>,
        stack: &HashMap<String, Variable>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::get_variable(id, stack, initialized)?);

        // res.push(PUT {pos: G});
        res.push(SWP { pos: G });
        res.push(READ);
        res.push(RSTORE { pos: G });

        initialized.insert(Self::get_name(&id));

        return Ok(res);
    }

    pub fn command_write(
        val: &Value,
        stack: &HashMap<String, Variable>,
        initialized: &mut HashSet<String>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::handle_value(val, stack, initialized)?);

        res.push(WRITE); // wyswietl A

        return Ok(res);
    }

    pub fn handle_value(
        val: &Value,
        stack: &HashMap<String, Variable>,
        initialized: &mut HashSet<String>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        match val {
            Value::Num { val } => {
                res.extend(Self::set_reg_a(*val));
            }
            Value::Var { val } => {
                res.extend(Self::get_variable(val, &stack, initialized)?);
                res.push(RLOAD { pos: A }); // A = wartość w komórce o numerze będącym w A
            }
        }

        return Ok(res);
    }

    pub fn get_variable(
        id: &Identifier,
        stack: &HashMap<String, Variable>,
        initialized: &HashSet<String>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        match id {
            Var { name } => {
                let variable = stack.get(&name.name);
                match variable {
                    Some(variable) => {
                        res.extend(Self::handle_variable_atomic(variable, name.clone())?)
                    }
                    None => {
                        return Err(CompilerError {
                            error_type: UndeclaredVariable,
                            id: name.name.clone(),
                            pos: name.begin,
                        });
                    }
                }
            }

            Array { name, var } => {
                // var is a num in this case
                let variable = stack.get(&name.name);
                match variable {
                    Some(variable) => {
                        res.extend(Self::handle_variable_array(variable, *var, name.clone())?)
                    }
                    None => {
                        return Err(CompilerError {
                            error_type: UndeclaredVariable,
                            id: name.name.clone(),
                            pos: name.begin,
                        });
                    }
                }
            }

            Array_Var { name, var } => {
                let index_var_option = stack.get(&var.name);
                let index_var;
                match index_var_option {
                    Some(index_var_value) => index_var = index_var_value,
                    None => {
                        return Err(CompilerError {
                            error_type: UndeclaredVariable,
                            id: name.name.clone(),
                            pos: name.begin,
                        });
                    }
                }

                if !initialized.contains(&var.name) {
                    return Err(CompilerError {
                        error_type: VariableNotInitialized,
                        id: var.name.clone(),
                        pos: var.begin,
                    });
                }

                let array_var = stack.get(&name.name);
                match array_var {
                    Some(array_var) => {
                        match array_var {
                            Variable::Atomic { position } => {
                                return Err(CompilerError {
                                    error_type: IncorrectUseOfVariable,
                                    id: name.name.clone(),
                                    pos: name.begin,
                                });
                            }
                            Variable::Array { position, lhs, rhs } => {
                                res.extend(Self::set_reg_a(*lhs)); // A = lhs wartosc 
                                res.push(SWP { pos: F }); // F = A, czyli lhs wartosc 
                                match index_var {
                                    Variable::Array { position, rhs, lhs } => {
                                        return Err(CompilerError {
                                            error_type: IncorrectUseOfVariable,
                                            id: var.name.clone(),
                                            pos: *position as usize,
                                        });
                                    }
                                    Variable::Atomic { position } => {
                                        res.push(LOAD {
                                            pos: *position as i64,
                                        });
                                    }
                                }
                                res.push(SUB { pos: F }); // A = F - A, czyyyli w A mamy offset!
                                res.push(SWP { pos: F }); // w F offset 
                                res.extend(Self::set_reg_a(*position));
                                res.push(ADD { pos: F });
                            }
                        }
                    }
                    None => {
                        return Err(CompilerError {
                            error_type: UndeclaredVariable,
                            id: name.name.clone(),
                            pos: name.begin,
                        });
                    }
                }
            }
        }

        return Ok(res);
    }

    pub fn handle_variable_array(
        var: &Variable,
        value: u64,
        name: PID,
    ) -> Result<Vec<Instruction>, CompilerError> {
        // array but num-indexed
        let mut res: Vec<Instruction> = vec![];

        match var {
            Variable::Atomic { position } => {
                return Err(CompilerError {
                    error_type: IncorrectUseOfVariable,
                    id: name.name,
                    pos: *position as usize,
                });
            }
            Variable::Array { position, lhs, rhs } => {
                if value > *rhs || value < *lhs {
                    return Err(CompilerError {
                        error_type: IndexOutOfBounds,
                        id: name.name,
                        pos: *position as usize,
                    });
                }
                let offset: u64 = value - lhs;
                res.extend(Self::set_reg_a(position + offset));
            }
        }

        return Ok(res);
    }

    pub fn handle_variable_atomic(
        var: &Variable,
        name: PID,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        match var {
            Variable::Atomic { position } => {
                res.extend(Self::set_reg_a(*position));
            }
            Variable::Array { position, lhs, rhs } => {
                return Err(CompilerError {
                    error_type: IncorrectUseOfVariable,
                    id: name.name,
                    pos: *position as usize,
                });
            }
        }

        return Ok(res);
    }

    pub fn command_if(
        cond: &Condition,
        comm: &Vec<Command>,
        else_comm: &Option<Vec<Command>>,
        initialized: &mut HashSet<String>,
        stack: &mut HashMap<String, Variable>,
        sp: &mut u64,
        procedures: &HashMap<String, ProcedureCompiler>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        let mut block_instructions: Vec<Instruction> = vec![];
        let mut else_block_instructions: Vec<Instruction> = vec![];

        block_instructions.extend(Self::handle_commands(
            comm,
            initialized,
            stack,
            sp,
            procedures,
        )?);

        match else_comm {
            Some(commands) => else_block_instructions.extend(Self::handle_commands(
                commands,
                initialized,
                stack,
                sp,
                procedures,
            )?),
            None => {}
        }

        match cond {
            Condition::Equal { l, r } => {
                res.extend(Self::if_handle_equal(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    &else_block_instructions,
                    initialized,
                )?);
            }
            Condition::NotEqual { l, r } => {
                res.extend(Self::if_handle_notequal(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    &else_block_instructions,
                    initialized,
                )?);
            }
            Condition::Greater { l, r } => {
                res.extend(Self::if_handle_greater(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    &else_block_instructions,
                    initialized,
                )?);
            }
            Condition::Less { l, r } => {
                res.extend(Self::if_handle_less(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    &else_block_instructions,
                    initialized,
                )?);
            }
            Condition::GreaterEqual { l, r } => {
                res.extend(Self::if_handle_greaterequal(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    &else_block_instructions,
                    initialized,
                )?);
            }
            Condition::LessEqual { l, r } => {
                res.extend(Self::if_handle_lessequal(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    &else_block_instructions,
                    initialized,
                )?);
            }
        }

        return Ok(res);
    }

    pub fn command_while(
        cond: &Condition,
        comm: &Vec<Command>,
        initialized: &mut HashSet<String>,
        stack: &mut HashMap<String, Variable>,
        sp: &mut u64,
        procedures: &HashMap<String, ProcedureCompiler>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        let mut block_instructions: Vec<Instruction> = vec![];
        block_instructions.extend(Self::handle_commands(
            comm,
            initialized,
            stack,
            sp,
            procedures,
        )?);

        match cond {
            Condition::Equal { l, r } => {
                res.extend(Self::while_handle_equal(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    initialized,
                )?);
            }
            Condition::NotEqual { l, r } => {
                res.extend(Self::while_handle_notequal(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    initialized,
                )?);
            }
            Condition::Greater { l, r } => {
                res.extend(Self::while_handle_greater(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    initialized,
                )?);
            }
            Condition::Less { l, r } => {
                res.extend(Self::while_handle_less(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    initialized,
                )?);
            }
            Condition::GreaterEqual { l, r } => {
                res.extend(Self::while_handle_greaterequal(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    initialized,
                )?);
            }
            Condition::LessEqual { l, r } => {
                res.extend(Self::while_handle_lessequal(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    initialized,
                )?);
            }
        }

        res.extend(block_instructions);
        res.push(JUMP {
            pos: -(res.len() as i64),
            adjust: true,
        });

        return Ok(res);
    }

    pub fn command_repeat(
        cond: &Condition,
        comm: &Vec<Command>,
        initialized: &mut HashSet<String>,
        stack: &mut HashMap<String, Variable>,
        sp: &mut u64,
        procedures: &HashMap<String, ProcedureCompiler>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];
        let mut conditions: Vec<Instruction> = vec![];

        let mut block_instructions: Vec<Instruction> = vec![];
        block_instructions.extend(Self::handle_commands(
            comm,
            initialized,
            stack,
            sp,
            procedures,
        )?);

        match cond {
            Condition::Equal { l, r } => {
                conditions.extend(Self::repeat_handle_equal(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    initialized,
                )?);
            }
            Condition::NotEqual { l, r } => {
                conditions.extend(Self::repeat_handle_notequal(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    initialized,
                )?);
            }
            Condition::Greater { l, r } => {
                conditions.extend(Self::repeat_handle_greater(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    initialized,
                )?);
            }
            Condition::Less { l, r } => {
                conditions.extend(Self::repeat_handle_less(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    initialized,
                )?);
            }
            Condition::GreaterEqual { l, r } => {
                conditions.extend(Self::repeat_handle_greaterequal(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    initialized,
                )?);
            }
            Condition::LessEqual { l, r } => {
                conditions.extend(Self::repeat_handle_lessequal(
                    l,
                    r,
                    stack,
                    &block_instructions,
                    initialized,
                )?);
            }
        }

        res.extend(block_instructions);
        res.extend(conditions);

        return Ok(res);
    }

    pub fn check_for_assignment(pid_for: &String, commands: &Vec<Command>) -> Option<usize> {

        for command in commands {

            match command {
                Command::Assign { name, expr } => match name {
                    Var { name } => {
                        if name.name == *pid_for {
                            return Some(name.begin);
                        }
                    }
                    Array_Var { name, var } => {
                        if name.name == *pid_for {
                            return Some(name.begin);
                        }
                    }
                    Array { name, var } => {
                        if name.name == *pid_for {
                            return Some(name.begin);
                        }
                    }
                },
                Command::If {
                    cond,
                    comm,
                    else_comm,
                } => {
                    let temp = Self::check_for_assignment(pid_for, comm);
                    match temp {
                        Some(result) => {
                            return Some(result);
                        }
                        None => {}
                    }

                    if else_comm.is_some() {
                        let temp_else = Self::check_for_assignment(pid_for, comm);
                        match temp_else {
                            Some(result) => {
                                return Some(result);
                            }
                            None => {}, // TODO!
                        }
                    }
                }
                Command::While { cond, comm } => {
                    let temp = Self::check_for_assignment(pid_for, comm);
                    match temp {
                        Some(result) => {
                            return Some(result);
                        }
                        None => {}
                    }
                }
                Command::Repeat { comm, cond } => {
                    let temp = Self::check_for_assignment(pid_for, comm);
                    match temp {
                        Some(result) => {
                            return Some(result);
                        }
                        None => {}
                    }
                }
                Command::For {
                    pid,
                    val_lhs,
                    val_rhs,
                    comm,
                    is_downto,
                } => {
                    let temp = Self::check_for_assignment(pid_for, comm);
                    match temp {
                        Some(result) => {
                            return Some(result);
                        }
                        None => {}
                    }
                    if pid.name == *pid_for {
                        return Some(pid.begin);
                    }
                }
                Command::Call { call } => {}, // TODO:
                Command::Read { name } => match name {
                    Var { name } => {
                        if name.name == *pid_for {
                            return Some(name.begin);
                        }
                    }
                    Array_Var { name, var } => {
                        if name.name == *pid_for {
                            return Some(name.begin);
                        }
                    }
                    Array { name, var } => {
                        if name.name == *pid_for {
                            return Some(name.begin);
                        }
                    }
                },
                Command::Write { val } => {}
            }
        }

        return None;
    }

    // TODO: check na wartosci
    // TODO: EXAMPLE A naprawic!
    pub fn command_for(
        pid: &String,
        val_lhs: &Value,
        val_rhs: &Value,
        comm: &Vec<Command>,
        is_downto: bool,
        initialized: &mut HashSet<String>,
        stack: &mut HashMap<String, Variable>,
        sp: &mut u64,
        procedures: &HashMap<String, ProcedureCompiler>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        let check = Self::check_for_assignment(pid, comm);
        match check {
            Some(position) => {
                return Err(CompilerError {
                    error_type: LoopIndexAssignment,
                    id: pid.clone(),
                    pos: position,
                });
            }
            None => {}
        }

        /* musimy jakos zainicjalizowac zmienna */

        // TODO: error ze nie moze to byc array
        stack.insert(pid.clone(), Variable::Atomic { position: *sp });
        let iterator_position: i64 = *sp as i64;
        *sp += 1;

        // obslugujemy tylko jeden case wiec:
        res.extend(Self::handle_value(&val_lhs, stack, initialized)?); // A == poczatkowa wartosc,
        // po ktorej iterujemy

        res.push(Instruction::STORE {
            pos: iterator_position,
        });
        initialized.insert(pid.clone());

        /* prawa strona - wartosc koncowa */
        let end_name: String = format!("{}:iter", pid.clone());
        stack.insert(end_name.clone(), Variable::Atomic { position: *sp });
        let end_value_position: i64 = *sp as i64;
        *sp += 1;

        // obslugujemy tylko jeden case wiec:
        res.extend(Self::handle_value(&val_rhs, stack, initialized)?); // A == wartosc koncowa,
        // ktora kopiujemy

        res.push(Instruction::STORE {
            pos: end_value_position,
        });
        initialized.insert(end_name.clone());

        let loop_begin: usize = res.len();

        let mut block_instructions: Vec<Instruction> = vec![];
        block_instructions.extend(Self::handle_commands(
            comm,
            initialized,
            stack,
            sp,
            procedures,
        )?);
        let block_instructions_len = block_instructions.len();

        if is_downto {
           res.push(Instruction::LOAD {
                pos: end_value_position,
            });

            res.push(Instruction::DEC { pos: A });
            res.push(Instruction::SWP { pos: E });
            res.push(Instruction::LOAD {
                pos: iterator_position,
            });

            res.push(Instruction::SUB { pos: E });
        } else {
            // A == iterator
            res.push(Instruction::LOAD {
                pos: iterator_position,
            });
            // E == iterator
            res.push(Instruction::SWP { pos: E });
            // A = wartosc koncowa
            // res.extend(Self::handle_value(&val_rhs, stack, initialized)?);
            res.push(Instruction::LOAD {
                pos: end_value_position,
            });
            // inc na wartosci koncowej
            res.push(Instruction::INC { pos: A });
            // jezeli teraz A bedzie ujemne, to znaczy, ze iterator jest za maly i musimy go
            // zwiekszyc o jeden i skoczyc na poczatek petli
            res.push(Instruction::SUB { pos: E });
        }

        res.push(Instruction::JZERO {
            pos: (block_instructions_len + 5) as i64,
            adjust: true,
        }); // + na inc

        res.extend(block_instructions);

        if is_downto {
            res.push(Instruction::LOAD {
                pos: iterator_position,
            });
            res.push(Instruction::DEC { pos: A });
            res.push(Instruction::STORE {
                pos: iterator_position,
            });
        } else {
            res.push(Instruction::LOAD {
                pos: iterator_position,
            });
            res.push(Instruction::INC { pos: A });
            res.push(Instruction::STORE {
                pos: iterator_position,
            });
        }
        res.push(Instruction::JUMP {
            pos: -((res.len() - loop_begin) as i64) as i64,
            adjust: true,
        });

        /* clearing the stack */
        stack.remove(pid);
        stack.remove(&end_name);
        *sp -= 2;
        initialized.remove(pid);
        initialized.remove(&end_name);

        return Ok(res);
    }

    pub fn command_call(
        call: &ProcCall,
        procedures: &HashMap<String, ProcedureCompiler>,
        initialized: &mut HashSet<String>,
        stack: &mut HashMap<String, Variable>,
        sp: &mut u64,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        // zapamietywanie rodzaju parametrów funkcji
        // I nie może być modyfikowany
        // O jest przyjmowany niezainicjalizowany
        // T to tablica

        let procedure_name: String = call.name.name.clone();
        let procedure_compiler_result = procedures.get(&procedure_name);
        let procedure_compiler: &ProcedureCompiler;
        match procedure_compiler_result {
            Some(pc) => procedure_compiler = pc,
            None => {
                return Err(CompilerError {
                    error_type: UndeclaredProcedure,
                    id: procedure_name,
                    pos: call.name.begin,
                });
            }
        }

        let procedure_declarations: Option<Declarations> = procedure_compiler.get_declarations();
        let call_arguments: Args = call.args.clone();
        let procedure_arguments: ArgsDecl = procedure_compiler.get_declared_arguments();

        // dla kazdej deklaracji z arg decl
        // sprawdz jej typ a nastepnie
        // sprawdz wszystkie komendy w ciele funkcji
        // i nastepnie czy jest git

        // TODO: czy to w ogole poprawnie się wywala?
        for arg_decl in &procedure_arguments {
            match arg_decl.type_name {
                Type::Array => {}

                Type::Const => {
                    for command in procedure_compiler.get_commands() {
                        match command {
                            Assign { name, expr } => match name {
                                Var { name } => {
                                    if (name.name == arg_decl.name.name) {
                                        return Err(CompilerError {
                                            error_type: CompilingErrorType::AssignmentToConstType,
                                            id: name.name,
                                            pos: name.begin,
                                        });
                                    }
                                }
                                Array_Var { name, var } => {
                                    if (name.name == arg_decl.name.name) {
                                        return Err(CompilerError {
                                            error_type: CompilingErrorType::AssignmentToConstType,
                                            id: name.name,
                                            pos: name.begin,
                                        });
                                    }
                                }
                                Array { name, var } => {
                                    if (name.name == arg_decl.name.name) {
                                        return Err(CompilerError {
                                            error_type: CompilingErrorType::AssignmentToConstType,
                                            id: name.name,
                                            pos: name.begin,
                                        });
                                    }
                                }
                            },

                            _ => {}
                        }
                    }
                }
                // TODO: co gdy jakby czytamy ją ale np w a := b?
                Type::Undefined => {
                    /*TODO*/
                    for command in procedure_compiler.get_commands() {
                        match command {
                            Read { name } => match name {
                                Var { name } => {
                                    if (name.name == arg_decl.name.name) {
                                        return Err(CompilerError {
                                            error_type: CompilingErrorType::AssignmentToConstType,
                                            id: name.name,
                                            pos: name.begin,
                                        });
                                    }
                                }
                                Array_Var { name, var } => {
                                    if (name.name == arg_decl.name.name) {
                                        return Err(CompilerError {
                                            error_type: CompilingErrorType::AssignmentToConstType,
                                            id: name.name,
                                            pos: name.begin,
                                        });
                                    }
                                }
                                Array { name, var } => {
                                    if (name.name == arg_decl.name.name) {
                                        return Err(CompilerError {
                                            error_type: CompilingErrorType::AssignmentToConstType,
                                            id: name.name,
                                            pos: name.begin,
                                        });
                                    }
                                }
                            },

                            _ => {}
                        }
                    }
                }
                Type::Scalar => {}
            }
        }

        // patrzę co z callami
        // jeżeli wywoływana procedura ma I, to nie mogę tam dać O ani T
        // jeżeli ma O, to nie mogę dam tać I ani T
        // jeżeli ma T, to nie mogę dać O ani I ani bez typu
        // jeżeli nie ma typu, to nie mogę T ani O

        // for command in procedure_compiler.get_commands() {
        //     match command {
        //         Call { call } => {
        //             // wyciągam procedurę, która jest wywoływana w ciele obecnej procedury
        //             let inside_call_name: String = call.name.name.clone();
        //             let call_compiler_result = procedures.get(&inside_call_name);
        //             let inside_call: &ProcedureCompiler;
        //             match call_compiler_result {
        //                 Some(pc) => inside_call = pc,
        //                 None => {
        //                     return Err(CompilerError {
        //                         error_type: UndeclaredProcedure,
        //                         id: inside_call_name,
        //                         pos: call.name.begin,
        //                     });
        //                 }
        //             }
        //
        //             // teraz jej ArgsDecl
        //             let inside_call_args_decl = inside_call.get_declared_arguments();
        //
        //             // porównuję typy
        //             for (arg_decl, call_arg) in inside_call_args_decl.iter().zip(call.args) {
        //                 match arg_decl.type_name {
        //                     Type::Array => {},
        //                     Type::Const => {
        //
        //
        //                     },
        //                     Type::Undefined => {},
        //                     Type::Scalar => {},
        //                 }
        //             }
        //         }
        //         _ => {}
        //     }
        // }

        if procedure_arguments.len() != call_arguments.len() {
            return Err(CompilerError {
                error_type: IncorrectNumberOfArguments,
                id: procedure_name,
                pos: call.name.begin,
            });
        }

        for arg in call_arguments.clone() {
            let arg_name: String = arg.name;
            let proc_arg_name: String = format!("@{}", procedure_name);

            if arg_name.contains(&proc_arg_name) {
                return Err(CompilerError {
                    error_type: RecursiveProcedureCall,
                    id: procedure_name,
                    pos: call.name.begin,
                });
            }
        }

        match procedure_declarations.clone() {
            Some(declarations) => {
                for variable in declarations {
                    match variable {
                        Declaration::Atomic { name } => {
                            stack.insert(
                                format!("{}@{}", name.name, procedure_name),
                                Variable::Atomic { position: *sp },
                            );

                            println!("SP: {}", sp);
                            *sp += 1;
                        }
                        Declaration::Array {
                            name,
                            num_lhs,
                            num_rhs,
                        } => {
                            stack.insert(
                                format!("{}@{}", name.name, procedure_name),
                                Variable::Array {
                                    position: *sp,
                                    lhs: num_lhs,
                                    rhs: num_rhs,
                                },
                            );
                            println!("SP: {}", sp);
                            *sp += num_rhs - num_lhs + 1;
                        }
                    }
                }
            }
            None => println!("Nothing declared!"),
        }

        // iterujemy po argumentach wywołania oraz argumentach z procedury
        for (argument, declared_argument) in call_arguments.iter().zip(procedure_arguments) {
            let mut variable_id: PID;
            let mut argument_id: PID;

            match procedure_declarations.clone() {
                Some(declarations) => {
                    for variable in declarations {
                        match variable {
                            Declaration::Atomic { name } => variable_id = name,
                            Declaration::Array {
                                name,
                                num_lhs,
                                num_rhs,
                            } => variable_id = name,
                        }

                        argument_id = declared_argument.name.clone();

                        if variable_id.name == argument_id.name {
                            return Err(CompilerError {
                                error_type: MultipleVariableDeclarations,
                                id: variable_id.name.clone(),
                                pos: variable_id.begin as usize,
                            });
                        }
                    }
                }
                None => println!("No procedure declaration"),
            }

            // TODO: this unwrap
            let pointer: &Variable = stack.get(&argument.name).unwrap();

            let declared_argument_name: String = declared_argument.name.name;

            match declared_argument.type_name.clone() {
                Type::Array => {
                    match pointer {
                        Variable::Atomic { position } => {
                            return Err(CompilerError {
                                error_type: IncorrectArgumentType,
                                id: declared_argument_name,
                                pos: *position as usize,
                            });
                        }
                        Variable::Array { position, lhs, rhs } => {
                            initialized.insert(argument.name.clone());
                            stack.insert(
                                format!("{}@{}", declared_argument_name, procedure_name), // TODO:
                                // is this correct?
                                Variable::Array {
                                    position: *position,
                                    lhs: *lhs,
                                    rhs: *rhs,
                                },
                            );
                            initialized
                                .insert(format!("{}@{}", declared_argument_name, procedure_name));
                        }
                    }
                }

                Type::Const => {
                    match pointer {
                        Variable::Atomic { position } => {
                            initialized.insert(argument.name.clone());

                            stack.insert(
                                format!("{}@{}", declared_argument_name, procedure_name),
                                Variable::Atomic {
                                    position: *position,
                                },
                            ); // TODO:
                            initialized
                                .insert(format!("{}@{}", declared_argument_name, procedure_name));
                        }
                        Variable::Array { position, lhs, rhs } => {
                            return Err(CompilerError {
                                error_type: IncorrectArgumentType,
                                id: declared_argument_name,
                                pos: *position as usize,
                            });
                        }
                    }
                }
                Type::Undefined => {
                    match pointer {
                        Variable::Atomic { position } => {
                            initialized.insert(argument.name.clone());

                            stack.insert(
                                format!("{}@{}", declared_argument_name, procedure_name),
                                Variable::Atomic {
                                    position: *position,
                                },
                            ); // TODO:
                            initialized
                                .insert(format!("{}@{}", declared_argument_name, procedure_name));
                        }
                        Variable::Array { position, lhs, rhs } => {
                            return Err(CompilerError {
                                error_type: IncorrectArgumentType,
                                id: declared_argument_name,
                                pos: *position as usize,
                            });
                        }
                    }
                }
                Type::Scalar => {
                    match pointer {
                        Variable::Atomic { position } => {
                            initialized.insert(argument.name.clone());

                            stack.insert(
                                format!("{}@{}", declared_argument_name, procedure_name),
                                Variable::Atomic {
                                    position: *position,
                                },
                            ); // TODO:
                            initialized
                                .insert(format!("{}@{}", declared_argument_name, procedure_name));
                        }
                        Variable::Array { position, lhs, rhs } => {
                            return Err(CompilerError {
                                error_type: IncorrectArgumentType,
                                id: declared_argument_name,
                                pos: *position as usize,
                            });
                        }
                    }
                }
            }
        }

        res.extend(Self::handle_commands(
            &procedure_compiler.get_commands(),
            initialized,
            stack,
            sp,
            procedures,
        )?);

        return Ok(res);
    }
}
