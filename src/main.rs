use std::env;

mod comandos;
mod parser;
mod persistencia;
mod store;

use store::Store;

///Punto de entrada de la aplicación.
///
///Inicializa el store en memoria y carga los datos persistidos,
///aplica el log de operaciones y ejecuta el comando recibido
///por el usuario.
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut store = Store::new();

    persistencia::cargar_data(&mut store);
    persistencia::aplicar_log(&mut store);

    comandos::ejecutar_comando(&args, &mut store);
}
