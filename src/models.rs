//use std::collections::HashMap;
use std::error::Error;


#[derive(PartialEq)]
#[derive(Debug)]
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
    //header: HashMap<String, String>,
    //body: Option<Vec<u8>>,
}


impl Request {
    
    pub fn get_method(request: &str) -> Result<Methods, Box<dyn Error>> {
          
        if let Some((first_line, _)) = request.split_once("\r\n") {

            let mut first_word = first_line.split_whitespace();
            
            if let Some(word) = first_word.next() {

                let method = match word {  
                    "GET" => Methods::GET,

                    "PATCH" => Methods::PATCH ,
                    
                    "POST" => Methods::POST,

                    "DELETE" => Methods::DELETE, 

                    _ => Methods::UNKNOWN,
                };

                return Ok(method);
            }
        } 
        Err("Failed to parse method".into())
        
    }

    pub fn get_path(request: &str) -> Result<String, Box<dyn Error>> {
        
        if let Some(( first_line, _)) = request.split_once("\r\n") {
            
            let mut words = first_line.split_whitespace();

            words.next().ok_or("Empty filed")?;

            let path = words.next().ok_or("Missing Field")?;
        
            return Ok(path.to_string());

        }

        Err(format!("Empty field").into())
    }

    pub fn get_host(request: &str) -> Result<String, Box<dyn Error>> {
       
       if let Some((_, other_lines)) = request.split_once("\r\n") {
             
           for lines in other_lines.lines() {
                if lines.starts_with("Host: ") {
                    if let Some(prefix) = lines.strip_prefix("Host: ") {
                        return Ok(prefix.to_string());
                    }
                } 
           }

        }
       
       Err("HOST NOT FOUND".into())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const DUMMY_REQUEST: &str = "GET /index.html HTTP/1.1\r\n
Host: localhost:8080\r\n
User-Agent: Mozilla/5.0\r\n
Accept: */*\r\n\r\n"; 

    #[test]
    fn test_method(){
        assert_eq!(Request::get_method(&DUMMY_REQUEST).unwrap(), Methods::GET) 
    }

    #[test]
    fn test_path() {
        assert_eq!(Request::get_path(&DUMMY_REQUEST).unwrap(), "/index.html")
    }

}       


