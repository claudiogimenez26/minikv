use crate::error::Error;
use crate::parser::parse_line;
use crate::store::Store;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

///Carga el snapshot inicial desde minikv.data.
pub fn cargar_data(store: &mut Store) {
    let file = File::open(".minikv.data");

    if let Ok(file) = file {
        let reader = BufReader::new(file);

        for l in reader.lines().map_while(Result::ok) {
            let mut parts = parse_line(&l);

            if parts.len() == 2 {
                let clave = parts.remove(0);
                let valor = parts.remove(0);

                store.set(clave, valor);
            }
        }
    }
}

///Aplica el log de operciones para reconstruir el estado actual.
pub fn aplicar_log(store: &mut Store) {
    let file = File::open(".minikv.log");

    if let Ok(file) = file {
        let reader = BufReader::new(file);

        for l in reader.lines().map_while(Result::ok) {
            let mut parts = parse_line(&l);

            if parts.len() >= 2 {
                let comando = parts.remove(0);

                if comando.as_str() == "set" {
                    if parts.len() == 2 {
                        let clave = parts.remove(0);
                        let valor = parts.remove(0);

                        store.set(clave, valor);
                    } else if parts.len() == 1 {
                        let clave = parts.remove(0);

                        store.delete(&clave);
                    }
                }
            }
        }
    }
}

///Guarda una operación set clave-valor en el log de operaciones.
pub fn guardar_set(clave: &str, valor: &str) {
    let mut log = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(".minikv.log")
    {
        Ok(f) => f,
        Err(e) => {
            Error::Output(format!("Error abriendo log: {}", e)).print();

            return;
        }
    };

    if let Err(e) = writeln!(log, "set \"{}\" \"{}\"", escapar(clave), escapar(valor)) {
        Error::Output(format!("Error escribiendo log: {}", e)).print();
    }
}

///Guarda una operacion unset (set sin valor) en el log de operaciones para indicar que la clave fue borrada.
pub fn guardar_delete(clave: &str) {
    let mut log = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(".minikv.log")
    {
        Ok(f) => f,
        Err(e) => {
            Error::Output(format!("Error abriendo log: {}", e)).print();

            return;
        }
    };

    if let Err(e) = writeln!(log, "set \"{}\"", escapar(clave)) {
        Error::Output(format!("Error escribiendo log: {}", e)).print();
    }
}

///genera un snapshot actual del estado del store guardando todas las claves y valores en minikv.data y luego trunca el log de operaciones.
pub fn ejecutar_snapshot(store: &Store) {
    let mut file = match File::create(".minikv.data") {
        Ok(f) => f,
        Err(e) => {
            Error::Output(format!("Error al crear el archivo de snapshot: {}", e)).print();
            return;
        }
    };

    for (clave, valor) in store.iter() {
        if let Err(e) = writeln!(file, "\"{}\" \"{}\"", escapar(clave), escapar(valor)) {
            Error::Output(format!("Error al escribir en snapshot: {}", e)).print();
            return;
        }
    }

    if let Err(e) = File::create(".minikv.log") {
        Error::Output(format!("Error al truncar log: {}", e)).print();
    }
}

///Escapa comillas para persistencia segura.
fn escapar(s: &str) -> String {
    s.replace('"', "\\\"")
}
