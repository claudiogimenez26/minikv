use std::env;

mod comandos;
mod error;
mod parser;
mod persistencia;
mod store;

use store::Store;

///punto de entrada de la aplicacion minikv.
///Fucionamiento
///1.leer argumentos de linea de comando
///2.inicializar el store en memoria
///3.cargar snapshot inicial desde archivo de data
///4.aplicar comandos del log para actualizar el estado
///5.ejecutar el comando ingresado por el usuario
/// #Archivos utilizados
/// - .minikv.data: snapshot inicial con pares clave-valor
/// - .minikv.log: log de operaciones para reconstruir el estado
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut store = Store::new();

    let data_path = ".minikv.data";
    let log_path = ".minikv.log";

    persistencia::cargar_data(data_path, &mut store);
    persistencia::aplicar_log(log_path, &mut store);

    comandos::ejecutar_comando(&args, &mut store, log_path);
}
