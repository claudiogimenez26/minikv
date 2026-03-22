use crate::error::Error;
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
            _ => Error::Command("Unknown command!".to_string()).print(),
        },
        None => Error::Input("No command provided!".to_string()).print(),
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

        (None, _) => Error::Input("No clave provided for set command!".to_string()).print(),
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
        None => Error::Input("No clave provided for get command!".to_string()).print(),
    }
}

///Ejecuta el comando length.
///Devuelve la cantidad de pares clave-valor almacenados.
fn ejecutar_length(store: &Store) {
    println!("{}", store.len());
}

#[cfg(test)]
mod comandos_tests {
    use super::*;
    use crate::store::Store;

    #[test]
    fn test_set_command() {
        let mut store = Store::new();

        let args = vec![
            "minikv".to_string(),
            "set".to_string(),
            "a".to_string(),
            "1".to_string(),
        ];

        ejecutar_comando(&args, &mut store);

        assert_eq!(store.get("a").unwrap(), "1");
    }

    #[test]
    fn test_unset_command() {
        let mut store = Store::new();

        let args_set = vec![
            "minikv".to_string(),
            "set".to_string(),
            "a".to_string(),
            "1".to_string(),
        ];

        ejecutar_comando(&args_set, &mut store);

        let args_unset = vec![
            "minikv".to_string(),
            "set".to_string(),
            "a".to_string(),
        ];

        ejecutar_comando(&args_unset, &mut store);

        assert!(store.get("a").is_none());
    }

    #[test]
    fn test_get_command() {
        let mut store = Store::new();

        store.set("x".to_string(), "10".to_string());

        let args = vec![
            "minikv".to_string(),
            "get".to_string(),
            "x".to_string(),
        ];

        ejecutar_comando(&args, &mut store);
    }
}
