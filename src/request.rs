use crate::models::Request;
use std::net::TcpStream;
use std::io::*;
use crate::cl_args;
 
// so here we create a function that reads streams to a buffer as bytes, 
//and then convert the bytes to string then parse the string to our request struct then, 
//call our methods on it to get the method the path and the host, 
//and the perform specific search based on our specified path gotten from the received request.


pub fn handle_request(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0u8; 4096];

    let incoming_streams = stream.read(&mut buffer[..])?;

    let bytes = &buffer[..incoming_streams];

    let converted_bytes = std::str::from_utf8(bytes).unwrap();

    let request = Request::new(converted_bytes);

    let extracted_request = request.unwrap();

    let _method = extracted_request.get_method();

    let _path = extracted_request.get_path();

    let _files = cl_args::get_args();

    //if method == "GET" {
        //if path == files {
            
        //} 
   // } 

    Ok(())
}
