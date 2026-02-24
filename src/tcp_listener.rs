use crate::models::Request;
use std::net::{TcpListener, TcpStream};
use std::error::Error;
use std::fs::*;
use std::io;
use std::io::Read;
 
type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>; 

async fn create_listener() -> GenericResult<()>{

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

fn handle_request(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0u8; 4096];

    let incoming_streams = stream.read(&mut buffer[..])?;

    let bytes = &buffer[..incoming_streams];

    Ok(())
}
