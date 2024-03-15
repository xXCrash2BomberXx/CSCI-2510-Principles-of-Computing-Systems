use std::net::{TcpStream, Shutdown};
use config::Config;
use std::process::exit;
use std::io::Write;

/* send_log - writes messages to the global logger
 *
 * Usage: 1. connect to logger and create log object with log_connect
 *        2. send log messages with log_send
 *        3. terminate logger connection with log_disconnect
 *
 * For example, in user code:
 *
 *  let logger = log_connect( "../config.toml" );
 *  log_send( &logger, "This is a log message to write to the global log." );
 *  log_send( &logger, "And another message to write to the global log." );
 *  log_disconnect( &logger );
 * */

//This should be considered an opaque type- the user should never
//interact with the internals of struct Log. See the usage idiom
//up above.
pub struct Log {
    stream: TcpStream,
}

pub fn log_connect( config_path: &str ) -> Log {
	let settings = match Config::builder()
		.add_source(config::File::with_name(config_path))
		.build() {
            Ok(x) => x,
            Err(x) => {println!("Could not open configuration file: {x}"); exit(-1)},
        }; 

	let log_ip = settings.get_string("log_ip").unwrap();
    let log_port = settings.get_string("log_port").unwrap();

    //Your code should create a connection to the logging server
    //by calling std::TcpStream::connect(), and if successful,
    //return it inside a Log struct.
    return match TcpStream::connect(log_ip.to_owned()+":"+&log_port.to_owned()) {
        Ok(x) => Log {stream: x},
        Err(x) => panic!("Could not connect to server: {x}"),
    };
}

pub fn log_disconnect( log: &Log ){
    //Your code should shut down the connection to the logging server
    //and do any other necessary cleanup.
    match log.stream.shutdown(Shutdown::Both) {
        Ok(_) => {},
        Err(x) => panic!("Could not shutdown server: {x}"),
    };
}

pub fn log_send( log: &mut Log, msg: &str ){
    //Your code should send the specified message to the server.
    //If an error occurs, it should print a descriptive error
    //message and call exit(-1)
    match log.stream.write_all(msg.as_bytes()) {
        Ok(_) => {
            match log.stream.flush() {
                Ok(_) => {},
                Err(x) => {println!("Could not flush message to the server: {x}"); exit(-1);},
            };
        },
        Err(x) => {println!("Could not send message to the server: {x}"); exit(-1);},
    };
}
