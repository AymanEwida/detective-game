pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CompilationError(String),
    LinkingError(String),
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error occurred, ").unwrap();
        
        match self {
            Self::CompilationError(message) => write!(f, "Compilation Error: {}", message),
            Self::LinkingError(message) => write!(f, "Linking Error {}", message),
        }
    }
}

impl std::error::Error for Error {}
