/// Tipos de errores posibles en el sistema MiniKV.
///
/// Este enum centraliza todos los errores que pueden ocurrir
/// durante la ejecución del programa:
/// - Errores de entrada del usuario (Input)
/// - Errores de salida o escritura (Output)
/// - Errores de comandos inválidos (Command)
pub enum Error {
    /// Error de entrada del usuario.
    Input(String),
    /// Error relacionado con la salida o escritura de datos.
    Output(String),
    /// Error relacionado con comandos inválidos o desconocidos.
    Command(String),
}

impl Error {
    ///Esta función se encarga de mostrar el mensaje de error al usuario, indicando el tipo de error (Input, Output o Command) y el mensaje específico.
    pub fn print(&self) {
        match self {
            Error::Input(msg) => println!("Input error: {}", msg),
            Error::Output(msg) => println!("Output error: {}", msg),
            Error::Command(msg) => println!("Command error: {}", msg),
        }
    }
}
