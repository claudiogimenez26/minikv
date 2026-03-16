use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::fs::OpenOptions;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("Arguments: {:?}", args);
    let mut store = HashMap::new();
    cargar_data(&mut store);
    aplicar_log(&mut store);    
    ejecutar_comando(&args, &mut store);
}

fn ejecutar_comando(args: &[String], store: &mut HashMap<String, String>) {
    match args.get(1){
        Some(cmd) => {
            match cmd.as_str() {
                "set" => ejecutar_set(args, store),
                "get" => ejecutar_get(args, store),
                //"length" => println!("comando, length!"),
                "length" => ejecutar_length(store),
                "snapshot" => ejecutar_snapshot(store),
                _ => println!("Unknown command: {}", cmd),
            }
        }
        None => println!("No command provided!"),
        
    }

        
}

fn ejecutar_set(args: &[String], store: &mut HashMap<String, String>) {
        println!("comando, set!");
        /* match args.get(2) {
            Some(clave) => {
                println!("Clave: {}", clave);
            }
            
            None => println!("No clave provided for set command!"),
        }
        match args.get(3) {
            Some(valor) => {
                println!("Valor: {}", valor);
            }
            
            None => println!("No valor provided for set command!"),
        } */
        match (args.get(2), args.get(3)) {
            (Some(clave), Some(valor)) => {
                println!("Clave: {}, Valor: {}", clave, valor);
                store.insert(clave.to_string(), valor.to_string());
                println!("Store: {:?}", store);
                
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

                if let Err(e) = writeln!(log, "set {} {}", clave, valor) {
                    println!("Error escribiendo log: {}", e);
                }
            }

            (Some(clave), None) => 
            //println!("No valor provided for set command!"),
            {
                store.remove(clave);
                println!("Clave '{}' eliminada del store.", clave);
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

                if let Err(e) = writeln!(log, "set {}", clave) {
                    println!("Error escribiendo log: {}", e);
                }
            }

            (None, _) => println!("No clave provided for set command!"),
        }
    }

    fn ejecutar_get(args: &[String], store: &mut HashMap<String, String>) {
        println!("comando, get!");
        /* match args.get(2) {
            Some(clave) => {
                println!("Clave: {}", clave);
            }
            
            None => println!("No clave provided for get command!"),
        } */
        match args.get(2) {
            Some(clave) => {
                //println!("Clave: {}", clave);
                match store.get(clave) {
                    Some(valor) => println!("Valor: {}", valor),
                    None => println!("Clave '{}' no encontrada en el store.", clave),
                }
            }
            
            None => println!("No clave provided for get command!"),
        }
    }

    fn ejecutar_length(store: &mut HashMap<String, String>) {
        println!("comando, length!");
        println!("Número de pares clave-valor en el store: {}", store.len());
    }

    fn cargar_data(store: &mut HashMap<String, String>) {
        let file = File::open(".minikv.data");
        //let reader = BufReader::new(file);
        
        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(l) = line {
                        //let parts: Vec<&str> = l.splitn(2, '=').collect();
                        let parts: Vec<&str> = l.split_whitespace().collect();
                        if parts.len() == 2 {
                            store.insert(parts[0].to_string(), parts[1].to_string());
                        }
                    }
                }
            }
            Err(e) => println!("Error al abrir el archivo de datos: {}", e),
        }
    }

    fn aplicar_log(store: &mut HashMap<String, String>) {
        let file = File::open(".minikv.log");
        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(l) = line {
                        let parts: Vec<&str> = l.split_whitespace().collect();
                        if parts.len() >= 2 {
                            match parts[0] {
                                "set" => {
                                    if parts.len() == 3 {
                                        store.insert(parts[1].to_string(), parts[2].to_string());
                                    }else if parts.len() == 2 {
                                        store.remove(parts[1]);
                                    }
                                }
                                _ => {},
                            }
                        }
                    }
                }
            }
            Err(_) => {}
            
        }
    }

    fn ejecutar_snapshot(store: &HashMap<String, String>) {
        println!("comando, snapshot!");
        let mut file = match File::create(".minikv.data") {
            Ok(f) => f,
            Err(e) => {
                println!("Error al crear el archivo de snapshot: {}", e);
                return;
            }
        };

        for (clave, valor) in store {
            if let Err(e) = writeln!(file, "{} {}", clave, valor) {
                println!("Error al escribir en el archivo de snapshot: {}", e);
                return;
            }
        }

        if let Err(e) = File::create(".minikv.log") {
        println!("Error al truncar el log: {}", e);
        }

        println!("Snapshot generado correctamente.");   
    }

    #[cfg(test)]
    mod tests {

        //use super::*;
        use std::collections::HashMap;

        #[test]
        fn test_insert_y_get() {

            let mut store = HashMap::new();

            store.insert("nombre".to_string(), "claudio".to_string());

            let valor = store.get("nombre").unwrap();

            assert_eq!(valor, "claudio");
        }

        #[test]
        fn test_delete_clave() {

            let mut store = HashMap::new();

            store.insert("edad".to_string(), "30".to_string());

            store.remove("edad");

            let valor = store.get("edad");

            assert!(valor.is_none());
        }

        #[test]
        fn test_length() {

            let mut store = HashMap::new();

            store.insert("a".to_string(), "1".to_string());
            store.insert("b".to_string(), "2".to_string());

            assert_eq!(store.len(), 2);
        }

    }
