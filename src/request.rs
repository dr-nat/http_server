use crate::models::Request;
use std::net::TcpStream;
use std::fs::*;
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

    let converted_bytes = std::str::from_utf8(bytes).expect("Failed to convert bytes to strings");

    let request = Request::new(converted_bytes);

    let extracted_request = request.expect("Failed to extract the request");

    let method = extracted_request.get_method();

    let path = extracted_request.get_path();

    let folder = cl_args::get_args();

    let extracted_folder = folder.expect("Failed to extract the Folder"); // so we ge the folder and then read through the directory, then compare if the path gotten is found and then return the file in the html format.

    if method.to_string() == "GET".to_string() {
       let workspace = read_dir(extracted_folder)?;
       
       for files in workspace {
            let file_entry = files?;
            let file_path = &file_entry.path();

            let file = file_entry.metadata();

            if file?.is_file().to_string() == path {
                let response_buffer = String::new(); 

                //let file_path = file?.path();

                let file_contents = std::fs::read_to_string(file_path)?; // so we get the file path and then read it to string and then format it using our function and then write it to the buffer we created. 

                let formated_response = format_string_response(file_contents);
                
                stream.write_all(response_buffer.as_bytes())?;
            }
        }
    } else {
        let not_found_response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";
        
        stream.write_all(not_found_response.as_bytes())?;

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
