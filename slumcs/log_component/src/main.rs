use config::Config;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, self};
use std::vec::Vec;

fn main() {
    let config_path = "../config.toml";

    let settings = match Config::builder()
        .add_source(config::File::with_name(config_path))
        .build() {
            Ok(x) => x,
            Err(x) => panic!("Could not open configuration file: {x}"),
        };

    let log_ip = settings.get_string("log_ip").unwrap();
    let log_port = settings.get_string("log_port").unwrap();
    let log_file = settings.get_string("log_file").unwrap();

    let listener = match TcpListener::bind(log_ip.to_owned()+":"+&log_port.to_owned()) {
        Ok(x) => x,
        Err(x) => panic!("Could not create listener: {x}"),
    };
    match listener.set_nonblocking(true) {
        Ok(_) => {},
        Err(x) => panic!("Error setting server to non-blocking: {x}"),
    };

    let mut clients: Vec<TcpStream> = Vec::new();
    loop {
        match listener.accept() {
            Ok((x, _)) => {
                println!("Added new client");
                match x.set_nonblocking(true) {
                    Ok(_) => {},
                    Err(x) => panic!("Error setting client to non-blocking: {x}"),
                };
                clients.push(x);
            },
            Err(x) if x.kind() == io::ErrorKind::WouldBlock => {},
            Err(x) => panic!("Error accepting socket connection: {x}"),
        };
        for mut client in &clients {
            let mut bytes = [0; 128];
            match client.read(&mut bytes) {
                Ok(0) => {},
                Ok(_) => {
                    match std::fs::File::options().append(true).create(true).open(log_file.clone()) {
                        Ok(mut x) => {
                            match x.write_all(std::str::from_utf8(&bytes).unwrap().trim().trim_matches(char::from(0)).as_bytes()) {
                                Ok(_) => {
                                    match x.flush() {
                                        Ok(_) => {println!("Logged new message");},
                                        Err(x) => panic!("Error flushing message to log file: {x}"),
                                    };
                                },
                                Err(x) => panic!("Error writing to log file: {x}"),
                            };
                        },
                        Err(x) => panic!("Could not open log file: {x}"),
                    }
                },
                Err(x) if x.kind() == io::ErrorKind::WouldBlock => {},
                Err(x) => panic!("Could not read from client: {x}"),
            };
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
