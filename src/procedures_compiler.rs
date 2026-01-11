/* procedures_compiler */

// TODO: is renaming correct?

use crate::ast::{Command::*, *};
use crate::instructions::Instruction::{self, *};

#[derive(Debug, Clone)]
pub struct ProcedureCompiler {
    pub name: String,
    declared_arguments: ArgsDecl, //TODO: type check?
    declarations: Option<Declarations>,
    commands: Commands,
}

impl ProcedureCompiler {
    pub fn get_declarations(&self) -> Option<Declarations> {
        return self.declarations.clone();
    }

    pub fn get_commands(&self) -> Commands {
        return self.commands.clone();
    }

    pub fn get_declared_arguments(&self) -> ArgsDecl {
        return self.declared_arguments.clone();
    }

    pub fn get_procedure_name(&self) -> String {
        return self.name.clone();
    }

    pub fn new(procedure: Procedure) -> Self {
        let renamed_commands =
            Self::rename_commands(&procedure.commands, &procedure.proc_head.name.name);
        println!(" Renamed tokens: ");
        println!("{:#?}\n", renamed_commands);

        Self {
            name: procedure.proc_head.name.name,
            declared_arguments: procedure.proc_head.args_decl,
            declarations: procedure.declarations,
            commands: renamed_commands,
        }
    }

    fn rename_commands(commands: &Commands, proc_name: &String) -> Commands {
        let mut renamed: Commands = vec![];

        for command in commands {
            match command {
                Assign { name, expr } => {
                    let renamed_name: Identifier = Self::rename_identifier(name, proc_name);
                    let renamed_expression: Expression = Self::rename_expression(expr, proc_name);
                    renamed.push(Command::Assign {
                        name: renamed_name,
                        expr: renamed_expression,
                    });
                }
                If {
                    cond,
                    comm,
                    else_comm,
                } => {
                    let renamed_conditions: Condition = Self::rename_condition(cond, proc_name);
                    let renamed_commands: Vec<Command> = Self::rename_commands(comm, proc_name);
                    let renamed_else_commands: Option<Vec<Command>>;
                    match else_comm {
                        Some(else_comm) => {
                            renamed_else_commands =
                                Some(Self::rename_commands(else_comm, proc_name))
                        }
                        None => renamed_else_commands = None,
                    };
                    renamed.push(Command::If {
                        cond: renamed_conditions,
                        comm: renamed_commands,
                        else_comm: renamed_else_commands,
                    });
                }
                While { cond, comm } => {
                    let renamed_conditions: Condition = Self::rename_condition(cond, proc_name);
                    let renamed_commands: Vec<Command> = Self::rename_commands(comm, proc_name);
                    renamed.push(Command::While {
                        comm: renamed_commands,
                        cond: renamed_conditions,
                    });
                }
                Repeat { comm, cond } => {
                    let renamed_conditions: Condition = Self::rename_condition(cond, proc_name);
                    let renamed_commands: Vec<Command> = Self::rename_commands(comm, proc_name);
                    renamed.push(Command::Repeat {
                        comm: renamed_commands,
                        cond: renamed_conditions,
                    });
                }
                For {
                    pid,
                    val_lhs,
                    val_rhs,
                    comm,
                    is_downto,
                } => {
                    // TODO:
                    //     println!("  For");
                    //     let res = Self::command_for(&pid, val_lhs, val_rhs, &comm, *is_downto, initialized, stack, sp);
                    //     ret.extend(res);
                }
                Call { call } => {
                    // TODO:
                    // let mut renamed_arguments: Args;
                    // let mut new_call: ProcCall;
                    //
                    // for argument in &call.args {
                    //         renamed_arguments.insert(format!("{}@{}", argument.name, self.name), argument.);
                    // }
                    //
                    //
                    // renamed.push(Command::Call {call: new_call});
                }
                Read { name } => {
                    let res = Self::rename_identifier(name, proc_name);
                    renamed.push(Command::Read { name: res });
                }
                Write { val } => {
                    let res = Self::rename_value(val, proc_name);
                    renamed.push(Command::Write { val: res });
                }
            }
        }
        return renamed;
    }

    fn rename_condition(condition: &Condition, proc_name: &String) -> Condition {
        let ret: Condition;

        match condition {
            Condition::Equal { l, r } => {
                let renamed_l = Self::rename_value(l, proc_name);
                let renamed_r = Self::rename_value(r, proc_name);
                ret = Condition::Equal {
                    l: renamed_l,
                    r: renamed_r,
                };
            }
            Condition::NotEqual { l, r } => {
                let renamed_l = Self::rename_value(l, proc_name);
                let renamed_r = Self::rename_value(r, proc_name);
                ret = Condition::NotEqual {
                    l: renamed_l,
                    r: renamed_r,
                };
            }
            Condition::Greater { l, r } => {
                let renamed_l = Self::rename_value(l, proc_name);
                let renamed_r = Self::rename_value(r, proc_name);
                ret = Condition::Greater {
                    l: renamed_l,
                    r: renamed_r,
                };
            }
            Condition::Less { l, r } => {
                let renamed_l = Self::rename_value(l, proc_name);
                let renamed_r = Self::rename_value(r, proc_name);
                ret = Condition::Less {
                    l: renamed_l,
                    r: renamed_r,
                };
            }
            Condition::GreaterEqual { l, r } => {
                let renamed_l = Self::rename_value(l, proc_name);
                let renamed_r = Self::rename_value(r, proc_name);
                ret = Condition::GreaterEqual {
                    l: renamed_l,
                    r: renamed_r,
                };
            }
            Condition::LessEqual { l, r } => {
                let renamed_l = Self::rename_value(l, proc_name);
                let renamed_r = Self::rename_value(r, proc_name);
                ret = Condition::LessEqual {
                    l: renamed_l,
                    r: renamed_r,
                };
            }
        }

        return ret;
    }

    fn rename_expression(expression: &Expression, proc_name: &String) -> Expression {
        let ret: Expression;

        match expression {
            Expression::Val { val } => {
                let renamed_value: Value = Self::rename_value(val, proc_name);
                ret = Expression::Val { val: renamed_value };
            }
            Expression::Add { l, r } => {
                let renamed_l: Value = Self::rename_value(l, proc_name);
                let renamed_r: Value = Self::rename_value(r, proc_name);
                ret = Expression::Add {
                    l: renamed_l,
                    r: renamed_r,
                };
            }
            Expression::Sub { l, r } => {
                let renamed_l: Value = Self::rename_value(l, proc_name);
                let renamed_r: Value = Self::rename_value(r, proc_name);
                ret = Expression::Sub {
                    l: renamed_l,
                    r: renamed_r,
                };
            }
            Expression::Mul { l, r } => {
                let renamed_l: Value = Self::rename_value(l, proc_name);
                let renamed_r: Value = Self::rename_value(r, proc_name);
                ret = Expression::Mul {
                    l: renamed_l,
                    r: renamed_r,
                };
            }
            Expression::Div { l, r } => {
                let renamed_l: Value = Self::rename_value(l, proc_name);
                let renamed_r: Value = Self::rename_value(r, proc_name);
                ret = Expression::Div {
                    l: renamed_l,
                    r: renamed_r,
                };
            }
            Expression::Mod { l, r } => {
                let renamed_l: Value = Self::rename_value(l, proc_name);
                let renamed_r: Value = Self::rename_value(r, proc_name);
                ret = Expression::Mod {
                    l: renamed_l,
                    r: renamed_r,
                };
            }
        }

        return ret;
    }
    fn rename_value(value: &Value, proc_name: &String) -> Value {
        let ret: Value;

        match value {
            Value::Num { val } => ret = Value::Num { val: *val },
            Value::Var { val } => {
                let temp_id: Identifier = Self::rename_identifier(val, proc_name);
                ret = Value::Var { val: temp_id };
            }
        }
        return ret;
    }

    fn rename_identifier(identifier: &Identifier, proc_name: &String) -> Identifier {
        let ret: Identifier;

        match identifier {
            Identifier::Var { name } => {
                let new_name: String = format!("{}@{}", name.name, proc_name);
                ret = Identifier::Var {
                    name: PID {
                        name: new_name,
                        begin: name.begin,
                        end: name.end,
                    },
                };
            }
            Identifier::Array_Var { name, var } => {
                let new_name: String = format!("{}@{}", name.name, proc_name);
                let new_var_name: String = format!("{}@{}", var.name, proc_name);
                ret = Identifier::Array_Var {
                    name: PID {
                        name: new_name,
                        begin: name.begin,
                        end: name.end,
                    },
                    var: PID {
                        name: new_var_name,
                        begin: name.begin,
                        end: name.end,
                    },
                };
            }
            Identifier::Array { name, var } => {
                let new_name: String = format!("{}@{}", name.name, proc_name);
                ret = Identifier::Array {
                    name: PID {
                        name: new_name,
                        begin: name.begin,
                        end: name.end,
                    },
                    var: *var,
                };
            }
        }

        return ret;
    }
}
