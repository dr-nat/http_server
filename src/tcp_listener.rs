use std::net::{TcpListener, TcpStream};
use std::error::Error;
use std::fs::*;
use std::io::*;
 
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

//pub fn handle_request(stream: TcpStream) {
  //  let mut stream_reader = BufReader::new(&stream);
    
//}
