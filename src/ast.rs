// TODO:
#![allow(non_camel_case_types)]
use std::usize;

pub type Num = u64;

#[derive(Debug, Clone)]
pub struct PID {
    pub name: String,
    pub begin: usize,
    pub end: usize,
}

// identifier -> pidentifier
// identifier -> pidentifier[pidentifier]
// identifier -> pidentifier[num]

#[derive(Debug, Clone)]
pub enum Identifier {
    Var { name: PID },
    Array_Var { name: PID, var: PID },
    Array { name: PID, var: Num },
}

// value -> num
// value -> identifier

#[derive(Debug, Clone)]
pub enum Value {
    Num { val: Num },
    Var { val: Identifier },
}

// condition -> value = value
// condition -> value != value
// condition -> value * value
// condition -> value > value
// condition -> value < value
// condition -> value >= value
// condition -> value <= value

#[derive(Debug, Clone)]
pub enum Condition {
    Equal { l: Value, r: Value },
    NotEqual { l: Value, r: Value },
    Greater { l: Value, r: Value },
    Less { l: Value, r: Value },
    GreaterEqual { l: Value, r: Value },
    LessEqual { l: Value, r: Value },
}

// expression -> value
// expression -> value + value
// expression -> value - value
// expression -> value * value
// expression -> value / value
// expression -> value % value

#[derive(Debug, Clone)]
pub enum Expression {
    Val { val: Value },
    Add { l: Value, r: Value },
    Sub { l: Value, r: Value },
    Mul { l: Value, r: Value },
    Div { l: Value, r: Value },
    Mod { l: Value, r: Value },
}

// args -> args, pidentifier
// args -> pidentifier

pub type Args = Vec<PID>;

// type -> T | I | O |

#[derive(Debug, Clone)]
pub enum Type {
    Array,
    Const,
    Undefined,
    Scalar,
}

// args_decl -> args_decl, type pidentifier
// args_decl -> type pidentifier

#[derive(Debug, Clone)]
pub struct ArgDecl {
    pub type_name: Type,
    pub name: PID,
}

pub type ArgsDecl = Vec<ArgDecl>;

// declarations -> declarations, pidentifier
// declarations -> declarations, pidentifier[num:num]
// declarations -> pidentifier
// declarations -> pidentifier[num:num]

#[derive(Debug, Clone)]
pub enum Declaration {
    Atomic {
        name: PID,
    },
    // TODO:
    Array {
        name: PID,
        num_lhs: Num,
        num_rhs: Num,
    },
}

pub type Declarations = Vec<Declaration>;

// proc_call -> pididentifier ( args )

#[derive(Debug, Clone)]
pub struct ProcCall {
    pub name: PID,
    pub args: Args,
}

// proc_head -> pididentifier ( args_decl )

#[derive(Debug)]
pub struct ProcHead {
    pub name: PID,
    pub args_decl: ArgsDecl,
}

// command -> identifier := expression;
// command -> IF condition THEN commands ELSE commands ENDIF
// command -> IF condition THEN commands ENDIF
// command -> WHILE condition DO commands ENDWHILE
// command -> REPEAT commands UNTIL condition;
// command -> FOR pidentifier FROM value TO value DO commands ENDFOR
// command -> FOR pidentifier FROM value DOWNTO value DO commands ENDFOR
// command -> proc_call;
// command -> READ identifier;
// command -> WRITE value;

#[derive(Debug, Clone)]
pub enum Command {
    Assign {
        name: Identifier,
        expr: Expression,
    },
    If {
        cond: Condition,
        comm: Commands,
        else_comm: Option<Commands>,
    },
    While {
        cond: Condition,
        comm: Commands,
    },
    Repeat {
        comm: Commands,
        cond: Condition,
    },
    // TODO:
    For {
        pid: PID,
        val_lhs: Value,
        val_rhs: Value,
        comm: Commands,
        is_downto: bool,
    },
    Call {
        call: ProcCall,
    },
    Read {
        name: Identifier,
    },
    Write {
        val: Value,
    },
}

// commands -> commands command
// commands -> command

pub type Commands = Vec<Command>;

// main -> PROGRAM IS declarations IN commands END
// main -> PROGRAM IS IN commands END

#[derive(Debug)]
pub struct Main {
    pub declarations: Option<Declarations>,
    pub commands: Commands,
}

// procedures -> procedures PROCEDURE proc_head IS declarations IN commands END
// procedures -> procedures PROCEDURE proc_head IS IN commands END
// <empty>

#[derive(Debug)]
pub struct Procedure {
    pub proc_head: ProcHead,
    pub declarations: Option<Declarations>,
    pub commands: Commands,
}

pub type Procedures = Vec<Procedure>;

// program_all -> procedures main

// in order to dump everything
#[derive(Debug)]
pub struct ProgramAll {
    pub procedures: Option<Procedures>,
    pub main: Main,
}
