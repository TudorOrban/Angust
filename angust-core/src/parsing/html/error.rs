
#[derive(Debug, Clone)]
pub enum ParsingError {
    ASTParsingError(String),
    ASTEvaluationError(String),

    FieldAccessError(String),

    InvalidDirectiveSyntax(String),
    InvalidDirective(String),
} 

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParsingError::ASTParsingError(msg) => write!(f, "AST parsing error: {}", msg),
            ParsingError::ASTEvaluationError(msg) => write!(f, "AST evaluation error: {}", msg),
            ParsingError::FieldAccessError(msg) => write!(f, "Property not found: {}", msg),
            ParsingError::InvalidDirectiveSyntax(msg) => write!(f, "Invalid directive syntax: {}", msg),
            ParsingError::InvalidDirective(msg) => write!(f, "Invalid directive: {}", msg),
        }
    }
}

impl std::error::Error for ParsingError {}