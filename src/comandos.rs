use crate::store::Store;
use crate::persistencia::{guardar_set, guardar_delete, ejecutar_snapshot};

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

fn ejecutar_get(args: &[String], store: &Store) {
    match args.get(2) {
        Some(clave) => match store.get(clave) {
            Some(valor) => println!("{}", valor),
            None => println!("NOT FOUND"),
        },
        None => println!("No clave provided for get command!"),
    }
}

fn ejecutar_length(store: &Store) {
    println!("{}", store.len());
}