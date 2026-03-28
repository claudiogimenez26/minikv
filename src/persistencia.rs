use crate::parser::parse_line;
use crate::store::Store;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

/// Carga el snapshot inicial desde un archivo de data.
///
/// # Parámetros
/// - `path`: ruta del archivo de snapshot (.minikv.data o test file)
/// - `store`: store en memoria donde se cargan los datos
///
/// # Comportamiento
/// - Si el archivo no existe, no hace nada
/// - Cada línea debe contener: "clave" "valor"
pub fn cargar_data(path: &str, store: &mut Store) -> bool {
    let file = File::open(path);

    if let Ok(file) = file {
        let reader = BufReader::new(file);

        for line in reader.lines().map_while(Result::ok) {
            let mut parts = parse_line(&line);

            if parts.len() == 2 {
                let clave = parts.remove(0);
                let valor = parts.remove(0);
                store.set(clave, valor);
            } else if !parts.is_empty() {
                return false;
            }
        }
    }

    true
}

/// Aplica el log de operaciones para reconstruir el estado.
///
/// # Parámetros
/// - `path`: ruta del archivo de log (.minikv.log o test file)
/// - `store`: store en memoria a reconstruir
///
/// # Formato soportado
/// - set "clave" "valor"
/// - set "clave"   (equivale a delete/unset)
pub fn aplicar_log(path: &str, store: &mut Store) -> bool {
    let file = File::open(path);
    if let Ok(file) = file {
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            let mut parts = parse_line(&line);
            if parts.is_empty() {
                continue;
            }
            let comando = parts.remove(0);
            if comando == "set" {
                if parts.len() == 2 {
                    let clave = parts.remove(0);
                    let valor = parts.remove(0);
                    store.set(clave, valor);
                } else if parts.len() == 1 {
                    let clave = parts.remove(0);
                    store.delete(&clave);
                } else {
                    false;
                }
            } else {
                false;
            }
        }
    }
    true
}

/// Guarda una operación SET en el log.
///
/// # Parámetros
/// - `path`: archivo de log (.minikv.log o test file)
/// - `clave`: clave a guardar
/// - `valor`: valor asociado
pub fn guardar_set(path: &str, clave: &str, valor: &str) {
    let mut log = match OpenOptions::new().create(true).append(true).open(path) {
        Ok(f) => f,
        Err(_) => {
            return;
        }
    };

    let _ = writeln!(log, "set \"{}\" \"{}\"", escapar(clave), escapar(valor));
}

/// Guarda una operación DELETE (unset) en el log.
///
/// # Parámetros
/// - `path`: archivo de log (.minikv.log o test file)
/// - `clave`: clave a eliminar
pub fn guardar_delete(path: &str, clave: &str) {
    let mut log = match OpenOptions::new().create(true).append(true).open(path) {
        Ok(f) => f,
        Err(_) => {
            return;
        }
    };

    let _ = writeln!(log, "set \"{}\"", escapar(clave));
}

/// Genera un snapshot del estado actual del store.
///
/// # Parámetros
/// - `path_data`: archivo de snapshot (.minikv.data o test file)
/// - `path_log`: archivo de log (.minikv.log o test file)
/// - `store`: estado actual en memoria
///
/// # Comportamiento
/// - Sobrescribe el snapshot
/// - Trunca el log
pub fn ejecutar_snapshot(path_data: &str, path_log: &str, store: &Store) {
    let mut file = match File::create(path_data) {
        Ok(f) => f,
        Err(_) => {
            return;
        }
    };

    for (clave, valor) in store.iter() {
        let _ = writeln!(file, "\"{}\" \"{}\"", escapar(clave), escapar(valor));
    }

    let _ = File::create(path_log);
}

/// Escapa comillas dobles para almacenamiento seguro.
///
/// # Ejemplo
/// ```
/// escapar(r#"hola"mundo"#);
/// // "hola\"mundo"
/// ```
fn escapar(s: &str) -> String {
    s.replace('"', "\\\"")
}

#[cfg(test)]
mod persistencia_tests {
    use super::*;
    use std::fs;

    fn cleanup(path: &str) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_cargar_data() {
        use std::fs::write;

        let path = ".test_cargar_data.data";

        cleanup(path);

        write(path, "\"a\" \"1\"\n\"b\" \"2\"").unwrap();

        let mut store = Store::new();

        cargar_data(path, &mut store);

        assert_eq!(store.get("a").unwrap(), "1");
        assert_eq!(store.get("b").unwrap(), "2");

        cleanup(path);
    }

    #[test]
    fn test_aplicar_log_set() {
        use std::fs::write;

        let path = ".test_aplicar_log_set.log";

        cleanup(path);

        write(path, "set \"a\" \"1\"\nset \"a\" \"2\"\n").unwrap();

        let mut store = Store::new();

        aplicar_log(path, &mut store);

        assert_eq!(store.get("a").unwrap(), "2");

        cleanup(path);
    }

    #[test]
    fn test_aplicar_log_delete() {
        use std::fs::write;

        let path = ".test_aplicar_log_delete.log";

        cleanup(path);

        write(path, "set \"a\" \"1\"\nset \"a\"\n").unwrap();

        let mut store = Store::new();

        aplicar_log(path, &mut store);

        assert!(store.get("a").is_none());

        cleanup(path);
    }

    #[test]
    fn test_snapshot() {
        use std::fs;

        let data = ".test.minikv.data";
        let log = ".test.minikv.log";

        let mut store = Store::new();
        store.set("a".to_string(), "1".to_string());
        store.set("b".to_string(), "2".to_string());

        ejecutar_snapshot(data, log, &store);

        let content = fs::read_to_string(data).unwrap();

        assert!(content.contains("\"a\" \"1\""));
        assert!(content.contains("\"b\" \"2\""));

        cleanup(data);
        cleanup(log);
    }
}
