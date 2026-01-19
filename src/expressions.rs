/* expression handling */

use crate::ast::{Expression::*, *};
use crate::compiler::Compiler;
use crate::errors::{CompilerError, CompilingErrorType::*};
use crate::helpers::Variable;
use crate::instructions::Instruction::{self, *};
use crate::instructions::Register::*;
use std::collections::{HashMap, HashSet};

impl Compiler {
    pub fn handle_expression(
        expression: &Expression,
        initialized: &mut HashSet<String>,
        stack: &HashMap<String, Variable>,
    ) -> Result<Vec<Instruction>, CompilerError> {
        let mut res: Vec<Instruction> = vec![];

        // TODO: zamienic z rst i add
        match expression {
            Val { val } => {
                Self::is_initialized(&val, initialized)?;

                res.extend(Self::handle_value(val, stack, initialized)?);
            }
            Add { l, r } => {
                Self::is_initialized(&l, initialized)?;
                Self::is_initialized(&r, initialized)?;

                res.extend(Self::handle_value(l, stack, initialized)?);
                res.push(SWP { pos: B });
                res.extend(Self::handle_value(r, stack, initialized)?);
                res.push(ADD { pos: B });
            }
            Sub { l, r } => {
                Self::is_initialized(&l, initialized)?;
                Self::is_initialized(&r, initialized)?;

                res.extend(Self::handle_value(r, stack, initialized)?);
                res.push(SWP { pos: B });
                res.extend(Self::handle_value(l, stack, initialized)?);
                res.push(SUB { pos: B });
            }
            Mul { l, r } => {
                Self::is_initialized(&l, initialized)?;
                Self::is_initialized(&r, initialized)?;

                res.extend(Self::handle_value(l, stack, initialized)?);
                res.push(SWP { pos: B });
                res.extend(Self::handle_value(r, stack, initialized)?);
                res.push(SWP { pos: C });
                res.extend(Self::handle_value(r, stack, initialized)?);
                res.push(SWP { pos: D });
                res.extend(Self::construct_multiplication());
            }
            Div { l, r } => {
                // TODO:
                Self::is_initialized(l, initialized)?;
                Self::is_initialized(r, initialized)?;

                res.extend(Self::handle_value(l, stack, initialized)?);
                res.push(SWP { pos: B });
                res.extend(Self::handle_value(r, stack, initialized)?);
                res.push(SWP { pos: C });
                res.extend(Self::construct_division());
            }
            Mod { l, r } => {
                // TODO
                Self::is_initialized(l, initialized)?;
                Self::is_initialized(r, initialized)?;

                res.extend(Self::handle_value(l, stack, initialized)?);
                res.push(SWP { pos: B });
                res.extend(Self::handle_value(r, stack, initialized)?);
                res.push(SWP { pos: C });
                res.extend(Self::construct_modulo());
            }
        }

        return Ok(res);
    }

    pub fn construct_multiplication() -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];
        // w B i C są już mnożone wartości
        res.push(RST { pos: A });
        res.push(SWP { pos: C });

        res.push(JZERO {
            pos: 19,
            adjust: true,
        });

        res.push(SHR { pos: A });
        res.push(SHL { pos: A });
        res.push(SWP { pos: D });
        res.push(SUB { pos: D });

        res.push(JPOS {
            pos: 5,
            adjust: true,
        });

        res.push(ADD { pos: D });
        res.push(SWP { pos: D });
        res.push(SWP { pos: C });

        res.push(JUMP {
            pos: 6,
            adjust: true,
        });

        res.push(ADD { pos: D });
        res.push(SWP { pos: D });
        res.push(INC { pos: A });
        res.push(SWP { pos: C });
        res.push(ADD { pos: B });
        res.push(SHL { pos: B });
        res.push(SHR { pos: D });
        res.push(SHR { pos: C });

        res.push(JUMP {
            pos: -19,
            adjust: true,
        });

        res.push(SWP { pos: C });

        return res;
    }

    pub fn construct_division() -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];
        // W B dzielna w C dzielnik

        res.push(RST { pos: A });
        res.push(SWP { pos: C });
        res.push(JZERO {
            pos: 30,
            adjust: true,
        });
        res.push(SWP { pos: C });
        res.push(SWP { pos: E });
        /*******/

        res.push(SWP { pos: E });
        res.push(RST { pos: D });
        res.push(SWP { pos: D });
        res.push(ADD { pos: C });
        res.push(SUB { pos: B });
        res.push(JPOS {
            pos: 21,
            adjust: true,
        });
        res.push(ADD { pos: C });
        res.push(SWP { pos: D });
        res.push(RST { pos: E });
        res.push(SWP { pos: E });
        res.push(ADD { pos: C });
        res.push(RST { pos: F });
        res.push(INC { pos: F });
        /*********************/
        res.push(SUB { pos: B });
        res.push(JPOS {
            pos: -14,
            adjust: true,
        });
        res.push(ADD { pos: D });
        res.push(SWP { pos: E });
        res.push(SWP { pos: B });
        res.push(SUB { pos: E });
        res.push(SWP { pos: B });
        res.push(ADD { pos: F });
        res.push(SHL { pos: D });
        res.push(SHL { pos: E });
        res.push(SHL { pos: F });
        res.push(SWP { pos: E });
        res.push(JUMP {
            pos: -12,
            adjust: true,
        });
        /*******/

        res.push(SWP { pos: D });
        /**********/

        return res;
    }

    // TODO:
    pub fn construct_modulo() -> Vec<Instruction> {
        let mut res: Vec<Instruction> = vec![];

        res.extend(Self::construct_division());
        res.push(Instruction::SWP { pos: B });

        return res;
    }
}
