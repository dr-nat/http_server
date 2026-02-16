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
    
    fn get_method(request: &str) -> Result<Methods, Box<dyn Error>> {
           
        if let Some((first_line, _)) = request.split_once("\r\n") {

            let mut first_word = first_line.split_whitespace();
            
            if let Some(word) = first_word.next() {

                let method = match word {  
                    "Get" => Methods::GET,

                    "Patch" => Methods::PATCH ,
                    
                    "Post" => Methods::POST,

                    "Delete" => Methods::DELETE, 

                    _ => Methods::UNKNOWN,
                };

                return Ok(method);
            }
        } 
        Err("Failed to parse method".into())
        
    }

    fn get_path(request: &str) -> Result<String, Box<dyn Error>> {
        
        if let Some((mut first_line, _)) = request.split_once("\r\n") {
            
            let mut words = first_line.split_whitespace();

            words.next().ok_or("Empty filed")?;

            let path = words.next().ok_or("Missing Field")?;
        
            return Ok(path.to_string());

        }

        Err(format!("Empty field").into())
    }
}

