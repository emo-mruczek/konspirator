/* procedures_compiler */

use crate::ast::{Command::*,  *};
use crate::instructions::Instruction::{self, *};
pub struct ProcedureCompiler {
    name: String,
    declared_arguments: ArgsDecl, //TODO: type check?
    declarations: Option<Declarations>,
    commands: Commands,
}

impl ProcedureCompiler {
    pub fn new(procedure: Procedure) -> Self {
        let renamed_commands =  Self::rename_commands(&procedure.commands);
        println!(" Renamed tokens: ");
        println!("{:#?}\n", renamed_commands); 

        Self {
            name: procedure.proc_head.name.name,
            declared_arguments: procedure.proc_head.args_decl,
            declarations: procedure.declarations,
            commands: renamed_commands,
        }
    }


    fn rename_commands(commands: &Commands)-> Commands {
        let mut renamed: Commands = vec![];

        for command in commands {
                
            match command {
                Assign {name, expr} => {
                    let renamed_name: Identifier = Self::rename_identifier(name); 
                    let renamed_expression: Expression = Self::rename_expression(expr);
                    renamed.push(Command::Assign { name: renamed_name, expr: renamed_expression });

                }
                If {cond, comm, else_comm} => {
                    let renamed_conditions: Condition = Self::rename_condition(cond);
                    let renamed_commands: Vec<Command> = Self::rename_commands(comm);
                    let renamed_else_commands: Option<Vec<Command>>;
                    match else_comm {
                        Some(else_comm) =>  renamed_else_commands = Some(Self::rename_commands(else_comm)),
                        None => renamed_else_commands = None,
                    };
                    renamed.push(Command::If{cond: renamed_conditions, comm: renamed_commands,else_comm: renamed_else_commands});
                },
                While {cond, comm} => {
                    let renamed_conditions: Condition = Self::rename_condition(cond);
                    let renamed_commands: Vec<Command> = Self::rename_commands(comm);
                    renamed.push(Command::While{comm: renamed_commands, cond: renamed_conditions});
                },
                Repeat {comm, cond} => {
                    let renamed_conditions: Condition = Self::rename_condition(cond);
                    let renamed_commands: Vec<Command> = Self::rename_commands(comm);
                    renamed.push(Command::Repeat{comm: renamed_commands, cond: renamed_conditions});
               
                },
                For {pid, val_lhs, val_rhs, comm, is_downto} => {
                //     println!("  For");
                //     let res = Self::command_for(&pid, val_lhs, val_rhs, &comm, *is_downto, initialized, stack, sp);
                //     ret.extend(res);
                }
                // // TODO
                Call {call} => println!("Call"),
                Read {name} => {
                    let res = Self::rename_identifier(name);
                    renamed.push(Command::Read {name: res});
                }
                Write {val} => {
                    let res = Self::rename_value(val);
                    renamed.push(Command::Write {val: res});
                }
        }
        }
        return renamed;
    }

    fn rename_condition(condition: &Condition) -> Condition {
        let ret: Condition;

        match condition {
            Condition::Equal{l, r} => {
                let renamed_l = Self::rename_value(l);
                let renamed_r = Self::rename_value(r);
                ret = Condition::Equal{l: renamed_l,r: renamed_r};
            }
            Condition::NotEqual{l, r} => {
                let renamed_l = Self::rename_value(l);
                let renamed_r = Self::rename_value(r);
                ret = Condition::NotEqual{l: renamed_l,r: renamed_r};
            }
            Condition::Greater{l, r} => {
                let renamed_l = Self::rename_value(l);
                let renamed_r = Self::rename_value(r);
                ret = Condition::Greater{l: renamed_l,r: renamed_r};
            }
            Condition::Less{l, r} => {
                let renamed_l = Self::rename_value(l);
                let renamed_r = Self::rename_value(r);
                ret = Condition::Less{l: renamed_l, r: renamed_r};
            }
            Condition::GreaterEqual{l, r} => {
                let renamed_l = Self::rename_value(l);
                let renamed_r = Self::rename_value(r);
                ret = Condition::GreaterEqual{l: renamed_l, r: renamed_r};
            }
            Condition::LessEqual{l, r} => {
                let renamed_l = Self::rename_value(l);
                let renamed_r = Self::rename_value(r);
                ret = Condition::LessEqual{l: renamed_l, r: renamed_r};
            }
        }

        return ret;
    }

    fn rename_expression(expression: &Expression) -> Expression {
        let ret: Expression;
        
        match expression {
            Expression::Val { val } => {
                let renamed_value: Value = Self::rename_value(val);
                ret = Expression::Val{val: renamed_value};
            },
            Expression::Add { l, r } => {
                let renamed_l: Value = Self::rename_value(l);
                let renamed_r: Value = Self::rename_value(r);
                ret = Expression::Add{l: renamed_l, r: renamed_r};

            },
            Expression::Sub { l, r } => {

                let renamed_l: Value = Self::rename_value(l);
                let renamed_r: Value = Self::rename_value(r);
                ret = Expression::Sub{l: renamed_l, r: renamed_r};
            },
            Expression::Mul { l, r } => {

                let renamed_l: Value = Self::rename_value(l);
                let renamed_r: Value = Self::rename_value(r);
                ret = Expression::Mul{l: renamed_l, r: renamed_r};
            },
            Expression::Div { l, r } => {

                let renamed_l: Value = Self::rename_value(l);
                let renamed_r: Value = Self::rename_value(r);
                ret = Expression::Div{l: renamed_l, r: renamed_r};
            },
            Expression::Mod { l, r } => {

                let renamed_l: Value = Self::rename_value(l);
                let renamed_r: Value = Self::rename_value(r);
                ret = Expression::Mod{l: renamed_l, r: renamed_r};
            },
        }

        return ret;
    }
    fn rename_value(value: &Value) -> Value {
        let ret: Value;

        match value {
            Value::Num {val} => ret  = Value::Num {val: *val},
            Value::Var {val} => {
                let temp_id: Identifier = Self::rename_identifier(val);
                ret = Value::Var {val: temp_id};
            }
        }
        return ret;
    }


    fn rename_identifier(identifier: &Identifier) -> Identifier {
        let ret: Identifier;

        match identifier {
            Identifier::Var { name } => {
                let new_name: String = format!("{}@{}@{}", name.name, name.begin, name.end);
                ret = Identifier::Var {name: PID {name: new_name, begin: name.begin, end: name.end}};
            },
            Identifier::Array_Var { name, var } => {
                let new_name: String = format!("{}@{}@{}", name.name, name.begin, name.end);
                let new_var_name: String = format!("{}@{}@{}", var.name, name.begin, name.end); // check
                ret = Identifier::Array_Var {name: PID {name: new_name, begin: name.begin, end: name.end}, var: PID{name: new_var_name, begin: name.begin, end: name.end }};
            },
            Identifier::Array { name, var } => {
                 let new_name: String = format!("{}@{}@{}", name.name, name.begin, name.end);
                ret = Identifier::Array {name: PID {name: new_name, begin: name.begin, end: name.end}, var: *var };

            },
        }

        return ret;
    }

   
}
