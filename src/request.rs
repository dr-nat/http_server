use crate::models::Request;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::fs::*;
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

    let extracted_folder = folder.expect("Failed to extract the Folder"); // so we ge the folder and then read through the directory, then compare if the path gotten is found and then return the file in the html format.

    if method.to_string() == "GET".to_string() {
       let workspace = read_dir(extracted_folder)?;

       // so readdir returns an iterator which is dir entry, so we use a for loop to get the direntry path of the result, and then call the direntry methods on it.
       
       for files in workspace {
            let file_entry = files?; // search why this line exist in your code
            let file_path = &file_entry.path();

            let file = file_entry.metadata();

            if file?.is_file().to_string() == path {
                let response_buffer = String::new(); 

                let file_contents = std::fs::read_to_string(file_path)?; // so we get the file path and then read it to string and then format it using our function and then write it to the buffer we created. 

                let formated_response = format_string_response(file_contents);
                println!("Sending Request:\n {:?}", formated_response);
                
                let _ = stream.write(response_buffer.as_bytes());
            }
        }
    } else {
        let not_found_response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";
        
        let _ = stream.write(not_found_response.as_bytes());

        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Request not found"));
    }

    Ok(())
}

fn format_string_response(file_input: String) -> String {
    
    let status_line = "HTTP1.1/ 200 OK";

    format!("{}\r\nContent-Length {}\r\nContent-Type: text/html\r\n\r\n{}", 
        status_line, 
        file_input.len(), 
        file_input)
}
