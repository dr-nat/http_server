use std::net::{TcpListener, TcpStream};
use std::error::Error;

pub fn create_listener() -> Result<(), Box<dyn Error>>{
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    
    for streams in listener.incoming() {
        match streams {
            Ok(stream) => {}
            Err(e) => {}
        }
    }

    Ok(())
}

