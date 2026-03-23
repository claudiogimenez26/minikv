use crate::error::Error;
use crate::persistencia::{ejecutar_snapshot, guardar_delete, guardar_set};
use crate::store::Store;

const DATA_PATH: &str = ".minikv.data";

/// Ejecuta el comando ingresado por el usuario.
///
/// # Parámetros
/// - `args`: argumentos de línea de comandos
/// - `store`: store en memoria
/// - `log_path`: ruta del archivo de log
pub fn ejecutar_comando(args: &[String], store: &mut Store, log_path: &str) {
    match args.get(1) {
        Some(cmd) => match cmd.as_str() {
            "set" => ejecutar_set(args, store, log_path),
            "get" => ejecutar_get(args, store),
            "length" => ejecutar_length(args, store),
            "snapshot" => {
                if args.len() > 2 {
                    Error::ExtraArgument.print();
                } else {
                    ejecutar_snapshot(DATA_PATH, log_path, store);
                    println!("OK");
                }
            }
            _ => Error::UnknownCommand.print(),
        },
        None => Error::MissingArgument.print(),
    }
}

/// Ejecuta el comando set.
/// - Si recibe clave y valor → guarda
/// - Si recibe solo clave → elimina
fn ejecutar_set(args: &[String], store: &mut Store, log_path: &str) {
    match (args.get(2), args.get(3)) {
        (Some(clave), Some(valor)) => {
            if args.len() > 4 {
                Error::ExtraArgument.print();
                return;
            }

            store.set(clave.to_string(), valor.to_string());
            guardar_set(log_path, clave, valor);
            println!("OK");
        }

        (Some(clave), None) => {
            if args.len() > 3 {
                Error::ExtraArgument.print();
                return;
            }

            store.delete(clave);
            guardar_delete(log_path, clave);
            println!("OK");
        }

        (None, _) => Error::MissingArgument.print(),
    }
}

/// Ejecuta el comando get.
/// Devuelve el valor o un error en formato pedido.
fn ejecutar_get(args: &[String], store: &Store) {
    match args.get(2) {
        Some(clave) => {
            // validar argumentos extra
            if args.len() > 3 {
                Error::ExtraArgument.print();
                return;
            }

            match store.get(clave) {
                Some(valor) => println!("{}", valor),
                None => Error::NotFound.print(),
            }
        }
        None => Error::MissingArgument.print(),
    }
}

/// Ejecuta el comando length.
/// Devuelve la cantidad de elementos.
fn ejecutar_length(args: &[String], store: &Store) {
    if args.len() > 2 {
        Error::ExtraArgument.print();
        return;
    }

    println!("{}", store.len());
}

#[cfg(test)]
mod comandos_tests {
    use super::*;
    use std::fs;

    fn cleanup(path: &str) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_set_command() {
        let mut store = Store::new();
        let log_path = ".test_set.log";

        cleanup(log_path);

        let args = vec![
            "minikv".to_string(),
            "set".to_string(),
            "a".to_string(),
            "1".to_string(),
        ];

        ejecutar_comando(&args, &mut store, log_path);

        assert_eq!(store.get("a").unwrap(), "1");

        cleanup(log_path);
    }

    #[test]
    fn test_unset_command() {
        let mut store = Store::new();
        let log_path = ".test_unset.log";

        cleanup(log_path);

        let args_set = vec![
            "minikv".to_string(),
            "set".to_string(),
            "a".to_string(),
            "1".to_string(),
        ];

        ejecutar_comando(&args_set, &mut store, log_path);

        let args_unset = vec!["minikv".to_string(), "set".to_string(), "a".to_string()];

        ejecutar_comando(&args_unset, &mut store, log_path);

        assert!(store.get("a").is_none());

        cleanup(log_path);
    }

    #[test]
    fn test_get_command() {
        let mut store = Store::new();
        let log_path = ".test_get.log";

        cleanup(log_path);

        store.set("x".to_string(), "10".to_string());

        let args = vec!["minikv".to_string(), "get".to_string(), "x".to_string()];

        ejecutar_comando(&args, &mut store, log_path);

        assert_eq!(store.get("x").unwrap(), "10");

        cleanup(log_path);
    }
}
