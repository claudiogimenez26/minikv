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
    pub fn print(&self) {
        match self {
            Error::NotFound => println!("ERROR: NOT FOUND"),
            Error::ExtraArgument => println!("ERROR: EXTRA ARGUMENT"),
            Error::InvalidDataFile => println!("ERROR: INVALID DATA FILE"),
            Error::InvalidLogFile => println!("ERROR: INVALID LOG FILE"),
            Error::MissingArgument => println!("ERROR: MISSING ARGUMENT"),
            Error::UnknownCommand => println!("ERROR: UNKNOWN COMMAND"),
        }
    }
}
