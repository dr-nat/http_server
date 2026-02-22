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
}


impl Request {
    pub fn new(request: &str) -> Result<Self, Box<dyn Error>>{

        //METHODS
        let mut lines = request.lines(); 

        let first_line = &lines.next().ok_or("first line not found")?;

        let mut words = first_line.split_whitespace();

        let first_word = words 
            .next()
            .ok_or("First word not found")?;


        let methods = match first_word {  
            "GET" => Methods::GET,

            "PATCH" => Methods::PATCH ,
            
            "POST" => Methods::POST,

            "DELETE" => Methods::DELETE, 

            _ => Methods::UNKNOWN,
        };
        
        //PATH
        let request_path = words.next().ok_or("Path not found")?; 
        
        //HOST
        let host = request.lines()
            .find(|line| line.starts_with("Host: "))
            .and_then(|line| line.strip_prefix("Host: "))
            .ok_or("Host header not found")?
            .to_string();
    

        Ok(Request{
            method: methods,
            host: host,
            path: request_path.to_string(),
        })
    }


    pub fn get_method(&self, request: &str) -> &Methods  {
        &self.method
    }

    pub fn get_path(&self, request: &str) -> String {
        self.path.clone()
    }

    pub fn get_host(&self, request: &str) -> String {
        self.host.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::models;

    const DUMMY_REQUEST: &str = "GET /index.html HTTP/1.1\r\n
Host: localhost:8080\r\n
User-Agent: Mozilla/5.0\r\n
Accept: */*\r\n\r\n"; 


    #[test]
    fn test_method(){
        let request: models::Request = Request{
            method: Methods::GET,
            host: "example.com".to_string(),
            path: "/index.html".to_string(),
        };


        assert_eq!(request.get_method(&DUMMY_REQUEST), &Methods::GET) 
    }

    #[test]
    fn test_path() {
        let request: models::Request = Request{
                method: Methods::GET,
            host: "example.com".to_string(),
            path: "/index.html".to_string(),
        };


        assert_eq!(request.get_path(&DUMMY_REQUEST), "/index.html")
    }


    #[test]
    fn get_host() {
        let request: models::Request = Request{
            method: Methods::GET,
            host: "example.com".to_string(),
            path: "/index.html".to_string(),
        };


        assert_eq!(request.get_host(&DUMMY_REQUEST), "localhost:8080")
    }
}       
