// TODO:

pub enum CompilingErrorType {
    UndeclaredVariable,
    MultipleVariableDeclarations,
    UndeclaredProcedure,
    MultipleProcedureDeclaration,
    RecursiveProcedureCall,
    // TODO: wrong I O T
    IncorrectUseOfVariable,
    IndexOutOfBounds,
    ArrayTypeVariableAsIndex,
    IncorrectTypeArgument,
    IncorrectNumberOfArguments,
    VariableNotInitialized,
}

pub struct CompilerError {
    pub error_type: CompilingErrorType,
    pub id: String,
    pub pos: usize,
}
