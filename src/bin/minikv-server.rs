use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};

use minikv::store::Store;
use minikv::comandos::process_line;

fn handle_client(mut stream: TcpStream, store: Arc<Mutex<Store>>,log_path: String) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut line = String::new();

    loop {
        line.clear();

        match reader.read_line(&mut line) {
            Ok(0) => break, // cliente cerró conexión
            Ok(_) => {
                let input = line.trim();

                // procesar comando
                let response = process_line(input, &store,&log_path);

                // responder al cliente
                if stream
                    .write_all(format!("{}\n", response).as_bytes())
                    .is_err()
                {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}

fn main() {
    // tomar dirección desde CLI
    let args: Vec<String> = std::env::args().collect();
    let addr = &args[1];

    // levantar server
    let listener = TcpListener::bind(addr).unwrap();
    println!("MiniKV server listening on {}", addr);

    // store compartido entre threads
    let store = Arc::new(Mutex::new(Store::new()));
    let log_path = ".minikv.log";
    // aceptar conexiones
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store = Arc::clone(&store);
                let log_path = log_path.to_string();

                thread::spawn(move || {
                    handle_client(stream, store, log_path);
                });
            }
            Err(_) => {
                eprintln!("ERROR: connection failed");
            }
        }
    }
}