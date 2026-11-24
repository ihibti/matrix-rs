use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatrixError {
    Singular,
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatrixError::Singular => write!(f, "matrix is singular (no inverse)"),
        }
    }
}

impl std::error::Error for MatrixError {}
