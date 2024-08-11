pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CompilationError(String),
    LinkingError(String),
    AttributeLocationError(String),
    LoadImageError(String),
    LoadFontFaceError(String),
    LoadCharacterError(String),
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error occurred, ").unwrap();
        
        match self {
            Self::CompilationError(message) => write!(f, "Compilation Error: {}", message),
            Self::LinkingError(message) => write!(f, "Linking Error: {}", message),
            Self::AttributeLocationError(message) => write!(f, "Attribute Location Error: {}", message),
            Self::LoadImageError(message) => write!(f, "Load Image Error: {}", message),
            Self::LoadFontFaceError(message) => write!(f, "Load Font Face Error: {}", message),
            Self::LoadCharacterError(message) => write!(f, "Load Character Error: {}", message),
        }
    }
}

impl std::error::Error for Error {}
