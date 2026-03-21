use crate::store::Store;
use crate::parser::parse_line;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

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

pub fn guardar_set(clave: &str, valor: &str) {
    let mut log = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(".minikv.log")
    {
        Ok(f) => f,
        Err(e) => {
            println!("Error abriendo log: {}", e);
            return;
        }
    };

    if let Err(e) = writeln!(
        log,
        "set \"{}\" \"{}\"",
        escapar(clave),
        escapar(valor)
    ) {
        println!("Error escribiendo log: {}", e);
    }
}

pub fn guardar_delete(clave: &str) {
    let mut log = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(".minikv.log")
    {
        Ok(f) => f,
        Err(e) => {
            println!("Error abriendo log: {}", e);
            return;
        }
    };

    if let Err(e) = writeln!(log, "set \"{}\"", escapar(clave)) {
        println!("Error escribiendo log: {}", e);
    }
}

pub fn ejecutar_snapshot(store: &Store) {
    
    let mut file = match File::create(".minikv.data") {
        Ok(f) => f,
        Err(e) => {
            println!("Error al crear el archivo de snapshot: {}", e);
            return;
        }
    };

    for (clave, valor) in store.iter() {
        if let Err(e) = writeln!(
            file,
            "\"{}\" \"{}\"",
            escapar(clave),
            escapar(valor)
        ) {
            println!("Error al escribir en snapshot: {}", e);
            return;
        }
    }

    // vaciar log
    if let Err(e) = File::create(".minikv.log") {
        println!("Error al truncar log: {}", e);
    }

}

fn escapar(s: &str) -> String {
    s.replace('"', "\\\"")
}