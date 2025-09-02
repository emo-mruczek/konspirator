// https://lalrpop.github.io/lalrpop/tutorial/005_building_asts.html

pub type Num = u64;

pub type PID = String;

pub enum Identifier {
    Basic {
        name: PID,
    },
    Array {
        name: PID,
        size: Num,
    },
    VLA {
        name: PID,
        size: PID,
    },
}

pub enum Value {
    Num {
        val: Num,
    },
    Var {
        val: Identifier, 
    },
}

pub enum Condition {
    Equal {
        l: Value,
        r: Value,
    },
    NotEqual {
        l: Value,
        r: Value,
    },
    Greater {
        l: Value,
        r: Value,
    },
    Less {
        l: Value,
        r: Value,
    },
    GreaterEqual {
        l: Value,
        r: Value,
    },
    LessEqual {
        l: Value,
        r: Value,
    },
}

pub enum Expression {
    Val {
        val: Value,
    },
    Add {
        l: Value,
        r: Value,
    },
    Sub {
        l: Value,
        r: Value,
    },
    Mul {
        l: Value,
        r: Value,
    },
    Div {
        l: Value,
        r: Value,
    },
    Mod {
        l: Value,
        r: Value,
    },
}

pub type Args = Vec<PID>;

pub enum ArgDecl {
    Basic {
        name: PID,
    },
    Tab {
        name: PID,
    },
}

pub type ArgsDecl = Vec<ArgDecl>;

pub enum Declaration {
    Basic {
        name: PID,
    },
    Array {
        name: PID,
        num: Num,
    },  
}

pub type Declarations = Vec<Declaration>;

pub struct ProcCall {
    pub name: PID,
    pub args: Args,
}

pub struct ProcHead {
    pub name: PID,
    pub args_decl: ArgsDecl,
}

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

pub type Commands = Vec<Command>;

// they dont have be any declarations
pub struct Main {
    pub declarations: Option<Declarations>,
    pub commands: Commands,
}

pub struct Procedure { 
    pub proc_head: ProcHead,
    pub declarations: Option<Declarations>,
    pub commands: Commands,
}

pub type Procedures = Vec<Procedure>;

// https://doc.rust-lang.org/std/option/ cuz can or cannot be (Some, None)
pub struct ProgramAll {
    pub procedures: Option<Procedures>,
    pub main: Main,
}
