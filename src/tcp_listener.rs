use std::net::{TcpListener, TcpStream};
use std::error::Error;
use std::fs::*;
use std::io;
use std::io::Read;
 
type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>; 

pub fn create_listener() -> GenericResult<()>{

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Waiting for request: -------");
    
    for streams in listener.incoming() {
        match streams {
            Ok(stream) => {}
            Err(e) => {}
        }
    }

    Ok(())
}

pub fn handle_request(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0u8; 4096];

    let num_of_bytes = stream.read(&mut buffer)?;

    let bytes = &buffer[..num_of_bytes];
    
    let converted_bytes = String::from_utf8_lossy(&bytes);

    let request = Request::new(converted_bytes)?;

    Ok(())
    
}
