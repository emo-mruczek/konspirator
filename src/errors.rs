// TODO:

pub enum CompilingErrorType {
    UndeclaredVariable,
    MultipleVariableDeclarations,
    UndeclaredProcedure,
    MultipleProcedureDeclaration,
    RecursiveProcedureCall,
    IncorrectUseOfVariable,
    IndexOutOfBounds,
    ArrayTypeVariableAsIndex,
    IncorrectArgumentType,
    AssignmentToConstType,
    IncorrectCallWithConst,
    CallWithScalarAsAnArray,
    CallWithArrayAsAScalar,
    IncorrectNumberOfArguments,
    VariableNotInitialized,
    LoopIndexAssignment,
}

pub struct CompilerError {
    pub error_type: CompilingErrorType,
    pub id: String,
    pub pos: usize,
}
