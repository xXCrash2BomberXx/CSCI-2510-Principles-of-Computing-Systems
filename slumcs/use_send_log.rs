use send_log::{log_connect, log_send, log_disconnect};

fn main(){
    //Connect to the logging server
    let mut log = log_connect( "../config.toml" );

    //Send a mesasge
    log_send( &mut log, "This is a test message for the global log." );
    log_send( &mut log, "This is a another test message." );

    log_disconnect( &mut log );
}
