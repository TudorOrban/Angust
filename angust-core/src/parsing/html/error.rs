
#[derive(Debug, Clone)]
pub enum ParsingError {
    ASTParsingError(String),
    ASTEvaluationError(String),
} 

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParsingError::ASTParsingError(msg) => write!(f, "AST parsing error: {}", msg),
            ParsingError::ASTEvaluationError(msg) => write!(f, "AST evaluation error: {}", msg),
        }
    }
}

impl std::error::Error for ParsingError {}