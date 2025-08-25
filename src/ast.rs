// https://lalrpop.github.io/lalrpop/tutorial/005_building_asts.html

pub type Num = i64;

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
        val: Val,
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

// TODO
pub enum Args {
    
}

// TODO:
pub enum Args_Decl {
    Basic {
        name: PID,
    },
    Tab {
        name: PID,
    },
}

// TODO:
pub enum Declarations {
    Basic {
        name: PID,
    },
    Array {
        name: PID,
        num: Num,
    },  
}

// TODO;
pub type Proc_Call {
    name: PID,
    
}

// TODO:
pub type Proc_Head {

}

// TODO
pub enum Command {
    Assign {
        name: Identifier,
        expr: Expression,
    },
    If {
        cond: Condition,
        comm: Commands,
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
        call: Proc_Call,
    },
    Read {
        name: Identifier,
    },
    Write {
        val: Value,
    },
}

// TODO:
pub enum Commands {

}

// they dont have be any declarations
pub enum Main {
    declarations: Option<Declarations>,
    commands: Commands,
}

// TODO:
pub enum Procedures {
    
}

// https://doc.rust-lang.org/std/option/ cuz can or cannot be (Some, None)
pub type Programm_All {
    procedures: Option<Procedures>,
    main: Main,
}
