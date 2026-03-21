use std::env;

mod store;
mod comandos;
mod persistencia;
mod parser;

use store::Store;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut store = Store::new();

    persistencia::cargar_data(&mut store);
    persistencia::aplicar_log(&mut store);

    comandos::ejecutar_comando(&args, &mut store);
}