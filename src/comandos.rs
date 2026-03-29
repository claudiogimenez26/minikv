use crate::error::Error;
use crate::persistencia::{ejecutar_snapshot, guardar_delete, guardar_set};
use crate::store::Store;
use std::sync::{Arc, Mutex};

const DATA_PATH: &str = ".minikv.data";

/// Ejecuta el comando ingresado por el usuario.
///
/// # Parámetros
/// - `args`: argumentos de línea de comandos
/// - `store`: store en memoria
/// - `log_path`: ruta del archivo de log
pub fn ejecutar_comando(args: &[String], store: &Arc<Mutex<Store>>, log_path: &str) -> String{
    match args.get(0) {
        Some(cmd) => match cmd.as_str() {
            "set" => ejecutar_set(args, store, log_path),
            "get" => ejecutar_get(args, store),
            "length" => ejecutar_length(args, store),
            "snapshot" => {
                if args.len() > 2 {
                    Error::ExtraArgument.to_string()
                } else {
                    let s = store.lock().unwrap();
                    ejecutar_snapshot(DATA_PATH, log_path, &*s);
                    "OK".to_string()
                }
            }
            _ => Error::UnknownCommand.to_string(),
        },
        None => Error::MissingArgument.to_string(),
    }
}

/// Ejecuta el comando set.
/// - Si recibe clave y valor → guarda
/// - Si recibe solo clave → elimina
fn ejecutar_set(
    args: &[String],
    store: &Arc<Mutex<Store>>,
    log_path: &str,
) -> String {
    match (args.get(1), args.get(2)) {
        (Some(clave), Some(valor)) => {
            if args.get(3).is_some() {
                return Error::ExtraArgument.to_string();
            }

            let mut s = store.lock().unwrap();
            s.set(clave.to_string(), valor.to_string());
            guardar_set(log_path, clave, valor);

            "OK".to_string()
        }

        (Some(clave), None) => {
            if args.get(2).is_some() {
                return Error::ExtraArgument.to_string();
            }

            
            let mut s = store.lock().unwrap();
            s.delete(clave);
            guardar_delete(log_path, clave);

            "OK".to_string()
        }

        (None, _) => Error::MissingArgument.to_string(),
    }
}

/// Ejecuta el comando get.
/// Devuelve el valor o un error en formato pedido.
fn ejecutar_get(args: &[String], store: &Arc<Mutex<Store>>) -> String {
    match args.get(1) {
        Some(clave) => {
            // validar argumentos extra
            if args.len() > 2 {
                return Error::ExtraArgument.to_string();
           } 

           let s = store.lock().unwrap(); 
           match s.get(clave) {
                Some(valor) => valor.to_string(),
                None => Error::NotFound.to_string(),
            }
        }
        None => Error::MissingArgument.to_string(),
    }
}

/// Ejecuta el comando length.
/// Devuelve la cantidad de elementos.
fn ejecutar_length(args: &[String], store: &Arc<Mutex<Store>>) -> String {
    if args.len() > 1 {
        return Error::ExtraArgument.to_string()
    } 
     
    let s = store.lock().unwrap();
    s.len().to_string()
}

pub fn process_line(input: &str, store: &Arc<Mutex<Store>>, log_path: &str,) -> String {
    let input = input.trim();
    if input.is_empty() {
        return crate::error::Error::UnknownCommand.to_string();
    }
    let args: Vec<String> = input
    .split_whitespace()
    .map(|s| s.to_string())
    .collect();
    ejecutar_comando(&args, store, log_path)
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
