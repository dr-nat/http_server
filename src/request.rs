use crate::models::Request;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
//use std::fs::*;
use crate::cl_args;
 
// so here we create a function that reads streams to a buffer as bytes, 
//and then convert the bytes to string then parse the string to our request struct then, 
//call our methods on it to get the method the path and the host, 
//and the perform specific search based on our specified path gotten from the received request.

pub async fn handle_request(mut stream: tokio::net::TcpStream) -> std::io::Result<()> { // so the function signature takes in a stream of type tcpstream of the tokio crate.
    let mut buffer = [0u8; 4096];

    let incoming_streams = stream.read(&mut buffer[..]).await?;

    let bytes = &buffer[..incoming_streams];

    let converted_bytes = std::str::from_utf8(bytes).expect("Failed to convert bytes to strings");

    let request = Request::new(converted_bytes);

    let extracted_request = request.expect("Failed to extract the request");

    let method = extracted_request.get_method();

    let path = extracted_request.get_path();

    println!("Request Method: {:?}, Path: {:?}", method, path);
    let folder = cl_args::get_args();

    let extracted_folder = folder.expect("Failed to extract the Folder"); 

    if method.to_string() == "GET".to_string() {

        // 1. Join the folder and the path (trimming the leading '/' from the URL)
        let clean_path = path.trim_start_matches('/');
        let full_path = std::path::Path::new(&extracted_folder).join(clean_path);

        // 2. Use match to decide exactly one outcome
        match full_path.exists() && full_path.is_file() {
            true => {
                let file_contents = tokio::fs::read(&full_path).await?;
                let formated_response = format_string_response(&file_contents);
                
                stream.write_all(&formated_response).await?;
                return Ok(()); 
            }
            false => {
                let not_found = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";
                stream.write_all(not_found.as_bytes()).await?;
                return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"));
            }
        }
    } 

    Ok(())
}

fn format_string_response(file_input: &[u8]) -> Vec<u8> {
    
    let status_line = "HTTP1.1/ 200 OK";

    let header = format!("{}\r\nContent-Length {}\r\nContent-Type: text/html\r\n\r\n", 
        status_line, 
        file_input.len(), 
    );

    let mut response = header.into_bytes();

    response.extend_from_slice(file_input);

    response
}
