use std::net::{Shutdown, TcpListener};
use std::io::{Read, Write};

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:23456") {
        Ok(x) => x,
        Err(x) => {println!("Could not create listener: {x}"); return},
    };

    let mut counter = 0;
    loop {
        let mut client = match listener.accept() {
            Ok((x,_)) => x,
            Err(x) => {println!("Error accepting socket connection: {x}"); return},
        };
        println!("Connected to client");

        counter = counter + 1;

        let mut bytes = [0; 128];
        match client.read(&mut bytes) {
            Ok(_) => {
                println!("Opening {:?}", std::str::from_utf8(&bytes).unwrap().trim().trim_matches(char::from(0)));
                match std::fs::File::open(std::str::from_utf8(&bytes).unwrap().trim().trim_matches(char::from(0))) {
                    Ok(mut x) => {
                        println!("Sending file contents to client");
                        let mut str: String = String::new();
                        x.read_to_string(&mut str);
                        client.write_all(str.as_bytes());
                    },
                    Err(x) => {
                        println!("{:?}", x);
                        client.write_all("Could not open file".as_bytes());
                    },
                }
            },
            Err(_) => {
                client.write_all("Could not read from client".as_bytes());
            },
        }
        client.flush();

        //Step 6: Close the connection
        match client.shutdown(Shutdown::Both) {
            Ok(_) => println!("Connection to client closed"),
            Err(x) => println!("Error during shutdown: {x}"),
        };
    }

}
