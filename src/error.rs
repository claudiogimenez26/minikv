/// Representa los errores posibles del sistema minikv,
/// siguiendo el formato requerido.
pub enum Error {
    NotFound,
    ExtraArgument,
    InvalidDataFile,
    InvalidLogFile,
    MissingArgument,
    UnknownCommand,
}

impl Error {
    /// Imprime el error en formato oficial:
    /// ERROR: {TIPO}
    pub fn to_string(&self) -> String {
        match self {
            Error::NotFound => "ERROR \"NOT FOUND\"".to_string(),
            Error::ExtraArgument => "ERROR \"EXTRA ARGUMENT\"".to_string(),
            Error::InvalidDataFile => "ERROR \"INVALID DATA FILE\"".to_string(),
            Error::InvalidLogFile => "ERROR \"INVALID LOG FILE\"".to_string(),
            Error::MissingArgument => "ERROR \"MISSING ARGUMENT\"".to_string(),
            Error::UnknownCommand => "ERROR \"UNKNOWN COMMAND\"".to_string(),
        }
    }
}
