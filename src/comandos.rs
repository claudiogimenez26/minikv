use crate::persistencia::{ejecutar_snapshot, guardar_delete, guardar_set};
use crate::store::Store;

///Ejecuta el comando ingresado por el usuario.
pub fn ejecutar_comando(args: &[String], store: &mut Store) {
    match args.get(1) {
        Some(cmd) => match cmd.as_str() {
            "set" => ejecutar_set(args, store),
            "get" => ejecutar_get(args, store),
            "length" => ejecutar_length(store),
            "snapshot" => ejecutar_snapshot(store),
            _ => println!("Unknown command: {}", cmd),
        },
        None => println!("No command provided!"),
    }
}

///Ejecuta el comando set.
///Si recibe clave y valor los guarda.
///Si recibe solo clave la borra.
fn ejecutar_set(args: &[String], store: &mut Store) {
    match (args.get(2), args.get(3)) {
        (Some(clave), Some(valor)) => {
            store.set(clave.to_string(), valor.to_string());
            guardar_set(clave, valor);

            println!("OK");
        }

        (Some(clave), None) => {
            store.delete(clave);
            guardar_delete(clave);

            println!("OK");
        }

        (None, _) => println!("No clave provided for set command!"),
    }
}

///Ejecuta el comnado get.
///Devuelve el valor asociado a la clave o "NOT FOUND" si no existe.
fn ejecutar_get(args: &[String], store: &Store) {
    match args.get(2) {
        Some(clave) => match store.get(clave) {
            Some(valor) => println!("{}", valor),
            None => println!("NOT FOUND"),
        },
        None => println!("No clave provided for get command!"),
    }
}


///Ejecuta el comando length.
///Devuelve la cantidad de pares clave-valor almacenados.
fn ejecutar_length(store: &Store) {
    println!("{}", store.len());
}
