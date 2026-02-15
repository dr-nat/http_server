use std::collections::HashMap;
use std::error::Error;

pub enum Methods {
    GET,
    PATCH,
    POST,
    DELETE,
    UNKNOWN
}


pub struct Request {
    method: Methods,
    host: String, 
    path: String,
    header: HashMap<String, String>,
    body: Option<Vec<u8>>,
}


impl Request {
    
    fn get_method(input: &str) -> Result<(), Box<dyn Error>> {
            
        if let Some((first_line, _)) = input.split_once("\r\n"){
            
            for method in first_line.split_whitespace() {
                match method {  
                    "Get" => Methods::GET,

                    "Patch" => Methods::PATCH ,
                    
                    "Post" => Methods::POST,

                    "Delete" => Methods::DELETE, 

                    _ => Methods::UNKNOWN,
                };
            }
        }
        
        Ok(())
    }
}

