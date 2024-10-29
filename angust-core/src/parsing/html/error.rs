
#[derive(Debug, Clone)]
pub enum ParsingError {
    ASTParsingError(String),
    ASTEvaluationError(String),

    FieldAccessError(String),

    InvalidDirectiveSyntax(String),
    InvalidDirective(String),

    InvalidDocument(String),
    InvalidTextElement(String),
} 

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParsingError::ASTParsingError(msg) => write!(f, "AST parsing error: {}", msg),
            ParsingError::ASTEvaluationError(msg) => write!(f, "AST evaluation error: {}", msg),

            ParsingError::FieldAccessError(msg) => write!(f, "Property not found: {}", msg),

            ParsingError::InvalidDirectiveSyntax(msg) => write!(f, "Invalid directive syntax: {}", msg),
            ParsingError::InvalidDirective(msg) => write!(f, "Invalid directive: {}", msg),

            ParsingError::InvalidDocument(msg) => write!(f, "Invalid document: {}", msg),
            ParsingError::InvalidTextElement(msg) => write!(f, "Invalid text element: {}", msg),
        }
    }
}

impl std::error::Error for ParsingError {}