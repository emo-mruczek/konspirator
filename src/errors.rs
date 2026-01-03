// TODO:

pub enum CompilingErrorType {
    undeclared_variable(String, usize),
    multiple_variable_declarations(String, usize),
    undeclared_procedure(String, usize),
    multiple_procedure_declaration(String, usize),
    recursive_procedure_call(String, usize),
    // TODO: wrong I O T
    // TODO: co? IncorrectUseOfVariable(String, usize),
    index_out_of_bounds(String, usize),
    array_type_variable_as_index(String, usize),
    incorrect_type_argument(String, usize),
    incorrect_number_of_arguments(String, usize),
}

impl CompilerError {
    // pub fn get_byte(&self) -> usize {
    //     match self {
    //         CompilerError::UndeclaredVariable(_, line) => *line,
    //         CompilerError::UndeclaredProcedure(_, line) => *line,
    //         CompilerError::IncorrectUseOfVariable(_, line) => *line,
    //         CompilerError::IndexOutOfBounds(_, line) => *line,
    //         CompilerError::ArrayUsedAsIndex(_, line) => *line,
    //         CompilerError::WrongArgumentType(_, line) => *line,
    //         CompilerError::DuplicateVariableDeclaration(_, line) => *line,
    //         CompilerError::DuplicateProcedureDeclaration(_, line) => *line,
    //         CompilerError::RecursiveProcedureCall(_, line) => *line,
    //         CompilerError::WrongNumberOfArguments(_, line) => *line,
    //     }
    // }
}
